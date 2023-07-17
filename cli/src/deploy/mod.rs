
use anyhow::{Result};
use crate::{subgraph::get_transaction_hash, cli::deploy::Consumer, deploy::registry::Fuji};
use self::{registry::{RainNetworks, Ethereum, Mumbai, Polygon}, transaction::get_transaction_data, dis::{DISpair, replace_dis_pair}}; 
use ethers::{providers::{Provider, Middleware, Http}} ; 
use ethers::{signers::LocalWallet, types::{Eip1559TransactionRequest, U64}, prelude::SignerMiddleware};
use std::{str::FromStr};
pub mod registry; 
use anyhow::anyhow; 

pub mod transaction; 
pub mod dis; 

pub async fn deploy_data(
    from_network : RainNetworks ,
    contract_address : String ,
    from_dis : DISpair , 
    to_dis : DISpair ,
    tx_hash : Option<String>
) -> Result<String> {    

    match tx_hash {
        Some(hash) => { 
            // Get tx data
            let tx_data = get_transaction_data(&from_network, &hash).await? ;  
            // Replace DIS instances 
            let tx_data = replace_dis_pair(&tx_data,&from_dis,&to_dis).unwrap() ;  

            Ok(tx_data)
        } ,
        None => {
            // Get tx hash
            let tx_data = get_transaction_hash(&from_network, &contract_address).await? ;  
            // Get tx data
            let tx_data = get_transaction_data(&from_network, &tx_data).await? ;  
            // Replace DIS instances 
            let tx_data = replace_dis_pair(&tx_data,&from_dis,&to_dis).unwrap() ;  

            Ok(tx_data)
        }
    }
    
}  


pub async fn deploy_contract(consumer : Consumer)  -> Result<()> {  

    if consumer.deploy { 

        // If deploy options is present then check if the private key was provided.
        let key = match consumer.private_key {
            Some(key) => key,
            None => return Err(anyhow!("\n ❌ Private Key Not Provided.\n Please provide unprefixed private key to deploy contract")),
        };   
        
        let data = deploy_data(
            consumer.origin_network ,
            consumer.contract_address, 
            DISpair::new(
                consumer.from_interpreter,
                consumer.from_store,
                consumer.from_deployer
            ) ,
            DISpair::new(
                consumer.to_interpreter,
                consumer.to_store,
                consumer.to_deployer
            ) ,
            consumer.transaction_hash
        ).await? ; 

        let (url,chain_id) = match consumer.to_network {
            RainNetworks::Ethereum => {
                (Ethereum::default().provider,Ethereum::default().chain_id)
            } ,
            RainNetworks::Polygon => {
                (Polygon::default().provider,Polygon::default().chain_id)
            },
            RainNetworks::Mumbai => {
                (Mumbai::default().provider,Mumbai::default().chain_id)
            },
            RainNetworks::Fuji => {
                (Fuji::default().provider,Fuji::default().chain_id)
            }
        } ; 
            
        let provider = Provider::<Http>::try_from(url)
        .expect("\n❌Could not instantiate HTTP Provider"); 

        let wallet: LocalWallet = key.parse()?; 
        let client = SignerMiddleware::new_with_provider_chain(provider, wallet).await?;  

        let bytes_data = ethers::core::types::Bytes::from_str(&data).unwrap() ; 
        let chain_id = U64::from_dec_str(&chain_id).unwrap() ; 
        let tx = Eip1559TransactionRequest::new().data(bytes_data).chain_id(chain_id) ; 

        let tx = client.send_transaction(tx, None).await?;   

        let receipt = tx.confirmations(6).await?.unwrap();  

        let print_str = format!(
            "{}{}{}{}{}" ,
            String::from("\nContract Deployed !!\n#################################\n✅ Hash : "),
            &serde_json::to_string_pretty(&receipt.transaction_hash).unwrap().to_string(), 
            String::from("\nContract Address: "),
            serde_json::to_string_pretty(&receipt.contract_address.unwrap()).unwrap(),
            String::from("\n-----------------------------------\n")
        ) ; 
        println!(
           "{}",
           print_str
        ) ;

        Ok(())

    }else{ 
        
        let tx_data = deploy_data(
                        consumer.origin_network ,
                        consumer.contract_address, 
                        DISpair::new(
                            consumer.from_interpreter,
                            consumer.from_store,
                            consumer.from_deployer
                        ) ,
                        DISpair::new(
                            consumer.to_interpreter,
                            consumer.to_store,
                            consumer.to_deployer
                        ) ,
                        consumer.transaction_hash
        ).await? ;

        println!("\n{}",tx_data) ;
        Ok(())

    }
     
}
