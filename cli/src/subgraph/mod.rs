use graphql_client::{GraphQLQuery, Response};
use reqwest;
use anyhow::{Result, Ok};
use crate::deploy::registry::{RainNetworks, Ethereum, Polygon, Mumbai, Fuji};

use anyhow::anyhow;


#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/subgraph/schema.json",
    query_path = "src/subgraph/query.graphql",
    response_derives = "Debug"
)]
pub struct ContractQuery;  
 


pub async fn get_transaction_hash( 
    network : &RainNetworks ,
    contract_address : &String
) -> Result<String> { 

    let variable = contract_query::Variables {
        addr: Some(contract_address.to_string()),
    };

    let request_body = ContractQuery::build_query(variable);
    let client = reqwest::Client::new(); 

    let url = match &network {
        RainNetworks::Polygon => {
            Polygon::default().url
        },  
        RainNetworks::Ethereum => {
            Ethereum::default().url
        }
        RainNetworks::Mumbai => {
            Mumbai::default().url
        },
        RainNetworks::Fuji => {
            String::from("")
        }
    } ; 
 
    let res: reqwest::Response = client
        .post(url)
        .json(&request_body)
        .send()
        .await?; 

    let response_body: Response<contract_query::ResponseData> = res.json().await?;  

    let query_contract = response_body
        .data.unwrap().contract ; 

    match query_contract {
        Some(contract_query) => {
            let tx_hash = contract_query.deploy_transaction.unwrap().id ;
            Ok(tx_hash) 
        } 
        None => { 
            let hash = get_scan_transaction_hash(network,contract_address).await? ; 
            Ok(hash)
        }
    } 
    
}  
 

pub async fn get_scan_transaction_hash(
    network : &RainNetworks ,
    contract_address : &String
) -> Result<String> {  

    let ( scan_url,  api_key) = match network {
        RainNetworks::Polygon => {
            (
                Polygon::default().block_scanner_api,
                Polygon::default().block_scanner_key,

            )
        },  
        RainNetworks::Ethereum => {
            (
                Ethereum::default().block_scanner_api,
                Ethereum::default().block_scanner_key,

            )
        }
        RainNetworks::Mumbai => {
            (
                Mumbai::default().block_scanner_api,
                Mumbai::default().block_scanner_key,

            )
        },
        RainNetworks::Fuji => {
            (
                Fuji::default().block_scanner_api,
                Fuji::default().block_scanner_key,

            )
        }
    } ;  

     let url = format!(
        "{}{}{}{}{}",
        scan_url,
        String::from("api?module=contract&action=getcontractcreation&contractaddresses="),
        contract_address,
        String::from("&apikey=") ,
        api_key
     ); 

     let res = reqwest::Client::new().get(url).send().await? ; 
     let body = res.text().await?;   

     let response_body: serde_json::Value = serde_json::from_str(&body).unwrap() ; 

    let tx_hash = match response_body.get("result").unwrap() {
        serde_json::Value::Array(val) => {
            val[0].get("txHash").unwrap() 
        } ,
        _ => { 
            return Err(anyhow!("\n❌ Contract not found.")) 
        }
    };  

    Ok(serde_json::from_value::<String>(tx_hash.clone()).unwrap())

}