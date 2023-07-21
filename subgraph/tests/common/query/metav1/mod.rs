use std::str::FromStr;
use anyhow::anyhow;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Url;
use rust_bigint::BigInt;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf as Bytes;
use web3::types::{Address, H160, U256};
use crate::common::wait::wait;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "tests/common/query/schema.json",
    query_path = "tests/common/query/metav1/query.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize"
)]

pub struct MetaV1;

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct MetaboardResponse {
    pub id: String,
    pub sender: Address,
    pub meta: String,
    pub subject: U256,
    pub magic_number: U256,
    pub payload: String,
    pub content_type: String,
    pub content_language: String,
    pub content_encoding: String,
    pub meta_board: Address,
}

impl MetaboardResponse {
    pub fn from(response: meta_v1::ResponseData) -> MetaboardResponse {
        let meta_v1 = response.meta_v1.unwrap();
        MetaboardResponse {
            id: String::from_utf8(meta_v1.id.to_vec()).unwrap(),
            sender: H160::from_str(&String::from_utf8(meta_v1.sender.to_vec()).unwrap()).unwrap(),
            meta: String::from_utf8(meta_v1.meta.to_vec()).unwrap(),
            subject: U256::from_dec_str(&meta_v1.subject.to_str_radix(16)).unwrap(),
            magic_number: U256::from_dec_str(&meta_v1.magic_number.to_str_radix(16)).unwrap(),
            payload: String::from_utf8_lossy(&meta_v1.payload.to_vec()).to_string(),
            content_type: meta_v1.content_type,
            meta_board: H160::from_str(&String::from_utf8(meta_v1.meta_board.id.to_vec()).unwrap())
                .unwrap(),
            content_language: meta_v1.content_language,
            content_encoding: meta_v1.content_encoding,
        }
    }
}

pub async fn get_metav1(meta_id: &str) -> anyhow::Result<MetaboardResponse> {
    wait().await?;

    let url = Url::from_str(&"http://localhost:8000/subgraphs/name/test/test")?;

    let variables = meta_v1::Variables {
        trx_hash: meta_id.to_string().into(),
    };

    let request_body = MetaV1::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(url.clone()).json(&request_body).send().await?;
    let response_body: Response<meta_v1::ResponseData> = res.json().await?;

    if let Some(meta_v1) = response_body.data.and_then(|data| Some(data)) {
        let resposne = MetaboardResponse::from(meta_v1);
        Ok(resposne)
    } else {
        return Err(anyhow!("Failed to get metaV1"));
    }
}
