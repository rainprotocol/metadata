use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::{fs::File, io::Write, process::Command};
use web3::{
    contract::Options,
    types::{Bytes, H160, U256},
};
use common::deploy::{deploy, Config};
use common::query::metaboard::get_meta_board;
use common::query::metav1::{get_metav1, MetaboardResponse};
use common::{deploy_meta_board, get_meta_board_from_address, get_web3, solidity_abi_v2};
mod common;

#[derive(Serialize, Deserialize)]
struct TestData {
    meta_board: H160,
    meta: String,
}
#[tokio::main]
#[test]
async fn deploy_and_emit_meta() -> anyhow::Result<()> {
    let meta_board = deploy_meta_board().await?;
    let deploy_block = get_web3()?.eth().block_number().await?;
    let sender = get_web3()?.eth().accounts().await?[1];
    let meta_board_address = format!("0x{}", hex::encode(meta_board.address()));

    let deploy_config = Config {
        contract_address: meta_board_address.clone(),
        block_number: deploy_block.as_u64(),
    };

    deploy(deploy_config).await?;

    let output = Command::new("bash")
    .args(&[
        "-c",
        "cargo run --manifest-path ../cli/Cargo.toml build -t json -E hex -i tests/abi.json -m solidity-abi-v2 -e deflate -l en",
    ])
    .output()
    .expect("Failed to get meta-hash");

    let meta_string;
    if output.status.success() {
        meta_string = String::from_utf8_lossy(&output.stdout).replace("0x", "");
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("{}", stderr));
    }

    meta_board
        .call(
            "emitMeta",
            (U256::from(1), Bytes(hex::decode(meta_string.clone())?)),
            sender,
            Options::default(),
        )
        .await?;

    let response = get_meta_board(meta_board_address.as_ref()).await?;

    assert_eq!(meta_board.address(), response.id);
    assert_eq!(meta_board.address(), response.address);
    assert_eq!(U256::one(), response.meta_count);
    for meta_v1 in response.metas.iter() {
        assert_eq!(meta_v1.replace("0x", ""), meta_string);
    }

    let test_data = TestData {
        meta_board: meta_board.address(),
        meta: meta_string,
    };

    let json_string = serde_json::to_string(&test_data)?;

    let file_path = "data.json";
    let mut file = File::create(file_path)?;
    file.write_all(json_string.as_bytes())?;

    Ok(())
}

#[tokio::main]
#[test]
async fn test_multi_emit_meta() -> anyhow::Result<()> {
    let file_path = "data.json";
    let mut file = File::open(file_path)?;
    let mut json_string = String::new();

    file.read_to_string(&mut json_string)?;

    let test_data: TestData = serde_json::from_str(&json_string)?;

    let meta_board = get_meta_board_from_address(test_data.meta_board);
    let meta_board_address = format!("0x{}", hex::encode(meta_board.address()));
    let sender = get_web3()?.eth().accounts().await?[1];

    for i in 0..5 {
        meta_board
            .call(
                "emitMeta",
                (
                    U256::zero() + i,
                    Bytes(hex::decode(test_data.meta.clone())?),
                ),
                sender,
                Options::default(),
            )
            .await?;
    }

    let response = get_meta_board(&meta_board_address).await?;
    assert_eq!(response.meta_count, U256::from(7));

    Ok(())
}

#[tokio::main]
#[test]
async fn test_meta_v1() -> anyhow::Result<()> {
    let file_path = "data.json";
    let mut file = File::open(file_path)?;
    let mut json_string = String::new();

    file.read_to_string(&mut json_string)?;

    let test_data: TestData = serde_json::from_str(&json_string)?;

    let meta_board = get_meta_board_from_address(test_data.meta_board);

    let sender = get_web3()?.eth().accounts().await?[1];

    meta_board
        .call(
            "emitMeta",
            (U256::one(), Bytes(hex::decode(test_data.meta.clone())?)),
            sender,
            Options::default(),
        )
        .await?;

    let resposne = get_metav1(test_data.meta.as_ref()).await?;

    let mut meta = test_data.meta.clone();
    meta.insert_str(0, "0x");
    let expected_id = meta.clone();
    let expected_meta = expected_id.clone();
    let expected_subject = U256::one();
    let expected_content_type = "application/json";
    let expected_content_language = "en";
    let expected_content_encoding = "deflate";
    let expected_sender = sender;
    let expected_magic_number = solidity_abi_v2();
    let expected_payload = resposne.payload.clone();
    let expected_meta_board = meta_board.address();

    let expected_response = MetaboardResponse{
        id: expected_id,
        sender: expected_sender,
        meta: expected_meta,
        subject: expected_subject,
        magic_number: expected_magic_number,
        payload: expected_payload,
        content_type: expected_content_type.to_string(),
        meta_board: expected_meta_board,
        content_encoding: expected_content_encoding.to_string(),
        content_language: expected_content_language.to_string()
    };

    assert_eq!(resposne, expected_response);

    Ok(())
}
