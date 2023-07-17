use anyhow::anyhow;
use common::get_web3;
use std::{process::Command};
use web3::{
    contract::Options,
    types::{Bytes, U256}
};

mod common;

#[tokio::main]
#[test]
async fn add_notice() -> anyhow::Result<()> {
    let meta_board = common::deploy_meta_board().await?;
    let sender = get_web3()?.eth().accounts().await?[1];

    let output = Command::new("bash")
    .args(&[
        "-c",
        "cargo run --manifest-path ../cli/Cargo.toml build -E hex -i tests/abi.json -m solidity-abi-v2 -t json -e deflate -l en",
    ])
    .output()
    .expect("Failed to get meta-hash");

    let meta: Bytes;
    if output.status.success() {
        // meta = format!("{}",String::from_utf8_lossy(&output.stdout));
        meta = Bytes(output.stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("{}", stderr));
    }
    
    let trx = meta_board.call("emitMeta", (U256::from(1), Bytes::from(meta)), sender, Options::default()).await?;
    println!("transaction : {:?}", trx);
    Ok(())
}
