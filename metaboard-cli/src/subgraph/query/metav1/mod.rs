use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use ethabi::ethereum_types::{H160, Address, U256};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Url;
use rust_bigint::BigInt;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf as Bytes;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/subgraph/query/schema.json",
    query_path = "src/subgraph/query/metav1/query.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize"
)]

pub struct MetaV1;

#[derive(Deserialize, Serialize)]
struct SubgraphResponse {
    id: String,
    sender: Address,
    meta: Bytes,
    subject: U256,
    magic_number: U256,
    payload: String,
    content_type: String,
    meta_board: Address,
}

impl SubgraphResponse {
    pub fn from(response: meta_v1::ResponseData) -> SubgraphResponse {
        let meta_v1 = response.meta_v1.unwrap();
        SubgraphResponse {
            id: meta_v1.id,
            sender: H160::from_str(&String::from_utf8(meta_v1.sender.to_vec()).unwrap()).unwrap(),
            meta: meta_v1.meta,
            subject: U256::from_dec_str(&meta_v1.subject.to_str_radix(16)).unwrap(),
            magic_number: U256::from_dec_str(&meta_v1.magic_number.to_str_radix(16)).unwrap(),
            payload: meta_v1.payload.replace("h'", "").replace("'", ""),
            content_type: meta_v1.content_type,
            meta_board: H160::from_str(&String::from_utf8(meta_v1.meta_board.id.to_vec()).unwrap()).unwrap(),
        }
    }
}

#[derive(Parser)]
pub struct MetaV1Struct {
    // subgraph api endpoint. if not given, local graph-node endpoint is used
    #[arg(
        short = 'e',
        long = "end_point",
        default_value = "http://localhost:8000/subgraphs/name/test/test"
    )]
    end_point: Option<String>,
    // metaboard contracts address
    #[arg(short = 't', long = "transaction_hash")]
    transaction_hash: Option<String>,
}

pub async fn query(build: MetaV1Struct) -> anyhow::Result<()> {
    let url = Url::from_str(&build.end_point.unwrap())?;
    
    let transaction_hash = build.transaction_hash.unwrap_or_else(|| Err(anyhow!("No transaction-hash provided")).unwrap());

    let variables = meta_v1::Variables {
        trx_hash: transaction_hash.into(),
    };

    let request_body = MetaV1::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(url.clone()).json(&request_body).send().await?;
    let response_body: Response<meta_v1::ResponseData> = res.json().await?;

    if let Some(meta_v1) = response_body.data.and_then(|data| Some(data)) {
        let resposne = SubgraphResponse::from(meta_v1);
        serde_json::to_writer(std::io::stdout(),&resposne)?;
    } else {
        tracing::warn!("Failed to get metaBard");
    }

    Ok(())
}
