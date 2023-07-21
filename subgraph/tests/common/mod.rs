pub mod deploy;
pub mod query;
pub mod wait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::str::FromStr;
use std::{fs::File, io::Read};
use web3::{
    contract::{Contract, Options},
    transports::Http,
    types::{H160, U256},
    Web3,
};
#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    from: String,
    data: String,
}

pub fn get_web3() -> anyhow::Result<Web3<Http>> {
    let web3 = web3::Web3::new(Http::new("http://localhost:8545")?);
    Ok(web3)
}

pub async fn deploy_meta_board() -> anyhow::Result<Contract<Http>> {
    // let url = "http://localhost:8545";
    let deployer = get_web3()?.eth().accounts().await?[0];

    let path = "abis/MetaBoard.json";
    let mut file = File::open(path).expect("No file");

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let abi_json: Value = serde_json::from_str(contents.as_str()).expect("Fail to parse JOSN");

    let bytecode = abi_json["bytecode"]["object"].as_str().unwrap();
    let contract = Contract::deploy(get_web3()?.eth(), abi_json["abi"].to_string().as_bytes())?;
    let contract = contract
        .confirmations(0)
        .options(Options::with(|opt| {
            opt.gas = Some(30_000_000.into());
        }))
        .execute(bytecode, (), deployer)
        .await?;

    Ok(contract)
}

pub fn get_meta_board_from_address(address: H160) -> Contract<Http> {
    let path = "abis/MetaBoard.json";
    let mut file = File::open(path).expect("No file");

    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    let abi_json: Value = serde_json::from_str(contents.as_str()).expect("Fail to parse JOSN");
    Contract::from_json(
        get_web3().unwrap().eth(),
        address,
        abi_json["abi"].to_string().as_bytes(),
    )
    .unwrap()
}

pub fn rain_meta_document_v1() -> U256 {
    U256::from_str("0xff0a89c674ee7874").unwrap()
}

pub fn solidity_abi_v2() -> U256 {
    U256::from_str("0xffe5ffb4a3ff2cde").unwrap()
}

pub fn op_meta_v1() -> U256 {
    U256::from_str("0xffe5282f43e495b4").unwrap()
}

pub fn interpreter_caller_meta_v1() -> U256 {
    U256::from_str("0xffc21bbf86cc199b").unwrap()
}