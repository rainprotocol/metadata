
use ethers::providers::{Provider, Middleware, Http} ;
use ethers::types::H256;
use std::str::FromStr;
use anyhow::Result;

use super::registry::{RainNetworks, Ethereum, Mumbai, Polygon, Fuji};

pub async fn get_transaction_data(from_network : &RainNetworks ,tx_hash : &String) -> Result<String> { 

    let url = match from_network {
        RainNetworks::Ethereum => {
            Ethereum::default().provider
        } ,
        RainNetworks::Polygon => {
            Polygon::default().provider
        },
        RainNetworks::Mumbai => {
            Mumbai::default().provider
        },
        RainNetworks::Fuji => {
            Fuji::default().provider
        }
    } ; 

    let provider = Provider::<Http>::try_from(url)?;  
    let h: H256 = H256::from_str(&tx_hash)?;  

    let tx_data = provider.get_transaction(h).await?.unwrap().input.to_string(); 
    Ok(tx_data) 

} 

