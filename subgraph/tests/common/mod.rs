pub mod deploy;
pub mod query;
pub mod wait;
use std::{fs::File, io::Read};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use web3::{transports::Http, Web3, contract::{ Contract, Options}};
#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    from: String,
    data: String,
}

pub fn get_web3() -> anyhow::Result<Web3<Http>> {
    let web3 = web3::Web3::new(Http::new("http://localhost:8545")?);
    Ok(web3)
}

pub async fn deploy_meta_board(
) -> anyhow::Result<Contract<Http>> {
    // let url = "http://localhost:8545";
    let deployer = get_web3()?.eth().accounts().await?[0];
    
    let path = "abis/MetaBoard.json";
    let mut file = File::open(path).expect("No file");

    let mut contents = String::new();
    let  _ = file.read_to_string(&mut contents);
    let abi_json: Value = serde_json::from_str(contents.as_str()).expect("Fail to parse JOSN");
    
    let bytecode = abi_json["bytecode"]["object"].as_str().unwrap();
    let contract = Contract::deploy(get_web3()?.eth(), abi_json["abi"].to_string().as_bytes())?;
    let contract = contract
    .confirmations(0)
    .options(Options::with(|opt| {
        opt.gas = Some(30_000_000.into());
    }))
    .execute(bytecode, (), deployer).await?;   

    
    Ok(contract)
}
