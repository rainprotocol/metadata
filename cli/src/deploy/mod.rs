
use anyhow::Result;
use crate::{subgraph::get_transaction_hash, cli::deploy::Consumer, deploy::registry::Fuji};
use self::{registry::{RainNetworks, Ethereum, Mumbai, Polygon}, transaction::get_transaction_data, dis::{DISpair, replace_dis_pair}}; 
use ethers::providers::{Provider, Middleware, Http} ; 
use ethers::{signers::LocalWallet, types::{Eip1559TransactionRequest, U64}, prelude::SignerMiddleware};
use std::str::FromStr;
pub mod registry; 
use anyhow::anyhow; 

pub mod transaction; 
pub mod dis; 

pub async fn get_deploy_data(
    from_network : RainNetworks ,
    contract_address : String ,
    from_dis : DISpair , 
    to_dis : DISpair ,
    tx_hash : Option<String>
) -> Result<String> {    

    match tx_hash {
        Some(hash) => { 
            // Get tx data
            let tx_data = get_transaction_data(from_network, hash).await? ;  
            // Replace DIS instances 
            let tx_data = replace_dis_pair(tx_data,from_dis,to_dis).unwrap() ;  

            Ok(tx_data)
        } ,
        None => {
            // Get tx hash
            let tx_data = get_transaction_hash(from_network, contract_address).await? ;  
            // Get tx data
            let tx_data = get_transaction_data(from_network, tx_data).await? ;  
            // Replace DIS instances 
            let tx_data = replace_dis_pair(tx_data,from_dis,to_dis).unwrap() ;  

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
        
        let data = get_deploy_data(
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
        
        let tx_data = get_deploy_data(
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


#[cfg(test)] 
mod test { 

    use super::get_deploy_data ; 
    use crate::deploy::transaction::get_transaction_data;
    use crate::deploy::registry::RainNetworks;
    use crate::deploy::DISpair;


    #[tokio::test]
    async fn test_rain_contract_deploy_data()  { 

        let from_network = RainNetworks::Mumbai ;   
        let contract_address = String::from("0x3cc6c6e888b4ad891eea635041a269c4ba1c4a63 ") ;  
        let tx_hash = None ; 

        let from_dis = DISpair {
            interpreter : Some(String::from("0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38")) ,
            store : Some(String::from("0xa5d9c16ddfd05d398fd0f302edd9e9e16d328796")) , 
            deployer : Some(String::from("0xd3870063bcf25d5110ab9df9672a0d5c79c8b2d5"))
        } ; 

        let to_dis = DISpair {
            interpreter : Some(String::from("0xfd1da7eee4a9391f6fcabb28617f41894ba84cdc")),
            store : Some(String::from("0x9b8571bd2742ec628211111de3aa940f5984e82b")),  
            deployer : Some(String::from("0x3d7d894afc7dbfd45bf50867c9b051da8eee85e9")),
        } ;   

        let tx_data = get_deploy_data(
            from_network,
            contract_address,
            from_dis,
            to_dis,
            tx_hash
        ).await.unwrap() ;

        let expected_tx_hash = String::from("0x13b9895c7eb7311bbb22ef0a692b7b115c98c957514903e7c3a0e454e3389378") ; 
        let expected_network = RainNetworks::Fuji ; 
        let expected_data = get_transaction_data(expected_network,expected_tx_hash).await.unwrap() ; 

        assert_eq!(tx_data,expected_data) ;

    }

     #[tokio::test]
    async fn test_non_rain_contract_deploy_data()  { 

        let from_network = RainNetworks::Mumbai ;   
        let contract_address = String::from("0x2c9f3204590765aefa7bee01bccb540a7d06e967") ;  
        let tx_hash = None ; 

        let from_dis = DISpair {
            interpreter : None,
            store : None,
            deployer : None,
        } ; 

        let to_dis = DISpair {
            interpreter : None,
            store : None,
            deployer : None,
        } ;   

        let tx_data = get_deploy_data(
            from_network,
            contract_address,
            from_dis,
            to_dis,
            tx_hash
        ).await.unwrap() ;

        let expected_tx_hash = String::from("0x2bcd975588b90d0da605c829c434c9e0514b329ec956375c32a97c87a870c33f") ; 
        let expected_network = RainNetworks::Fuji ; 
        let expected_data = get_transaction_data(expected_network,expected_tx_hash).await.unwrap() ; 

        assert_eq!(tx_data,expected_data) ;

    }

    #[tokio::test]
    async fn test_tx_hash_deploy_data()  { 

        let from_network = RainNetworks::Mumbai ;   
        let contract_address = String::from("0x5f02c2f831d3e0d430aa58c973b8b751f3d81b38 ") ;  
        let tx_hash = Some(String::from("0xd8ff2d9381573294ce7d260d3f95e8d00a42d55a5ac29ff9ae22a401b53c2e19")) ; 

        let from_dis = DISpair {
            interpreter : None,
            store : None,
            deployer : None,
        } ; 

        let to_dis = DISpair {
            interpreter : None,
            store : None,
            deployer : None,
        } ;   

        let tx_data = get_deploy_data(
            from_network,
            contract_address,
            from_dis,
            to_dis,
            tx_hash
        ).await.unwrap() ;

        let expected_tx_hash = String::from("0x15f2f57f613a159d0e0a02aa2086ec031a2e56e0b9c803d0e89be78b4fa9b524") ; 
        let expected_network = RainNetworks::Fuji ; 
        let expected_data = get_transaction_data(expected_network,expected_tx_hash).await.unwrap() ; 

        assert_eq!(tx_data,expected_data) ;

    } 

}
