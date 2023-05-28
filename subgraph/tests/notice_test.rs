use ethers::{
    providers::Middleware,
    prelude::{SignerMiddleware, abigen},
};
use eyre::Context;
use serde_json::Value;

use std::fs::File;
use std::io::Read;

use ethers::core::types::{ U256, Bytes};

use crate::common::cbor::{encode_cbor, MagicNumbers};
mod common;

abigen!(MetaBoard, "abis/MetaBoard.json");

#[tokio::main]
#[test]
async fn add_notice() -> Result<(), eyre::ErrReport> {
    let provider = common::get_rpc_provider("http://localhost:8545");
    
    assert_eq!(
        U256::from(31337),
        provider.get_chainid().await.unwrap()
    );

    let project = common::compile("contracts/").await?;
    let signers = common::get_signers()?;

    let deployer = signers[2].clone();

    let meta_board = common::deploy_meta_board(project, deployer).await?;

    let emitter = signers[3].clone();
    let emitter_client = SignerMiddleware::new(provider.clone(), emitter).into();

    let contract = MetaBoard::new(meta_board.address(), emitter_client);

    let path = "abis/MetaBoard.json";
    let mut file = File::open(path).expect("No file");

    let mut contents = String::new();
    let  _ = file.read_to_string(&mut contents);
    let abi: Value = serde_json::from_str(contents.as_str()).expect("Fail to parse JOSN");
    // println!("ABI {}", abi["abi"].to_string());
    let payload = abi["abi"].to_string();
    let magic_number = MagicNumbers::rain_meta_document();
    let content_type = "application/json".to_string();
    let encoded_data = encode_cbor(payload, magic_number, content_type, None, None);
    println!("{:?}", encoded_data);
    // contract.emit_meta(U256::from(1), encoded_data).send().await?;


    Ok(())
}

