use std::{any, str::FromStr};

use anyhow::anyhow;
use graphql_client::{GraphQLQuery, Response};
use web3::types::{Address, U256, H160};
use reqwest::Url;
use rust_bigint::BigInt;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf as Bytes;

use self::meta_board::ResponseData;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "tests/common/query/schema.json",
    query_path = "tests/common/query/metaboard/metaboard.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize"
)]

pub struct MetaBoard;

pub struct MetaBoardStruct {
    // subgraph api endpoint. if not given, local graph-node endpoint is used
    end_point: Option<String>,
    // metaboard contracts address
    meta_board_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MetaV1Response {
    id: Address,
    address: Address,
    meta_count: U256,
    metas: Vec<String>,
}

impl MetaV1Response {
    pub fn from(response: ResponseData) -> MetaV1Response {
        let meta_board = response.meta_board.unwrap();
        let metas = meta_board.metas.unwrap();

        MetaV1Response {
            id: H160::from_str(&String::from_utf8(meta_board.id.to_vec()).unwrap()).unwrap(),
            address: H160::from_str(&String::from_utf8(meta_board.address.to_vec()).unwrap()).unwrap(),
            meta_count: U256::from_dec_str(&meta_board.meta_count.to_str_radix(16)).unwrap(),
            metas: metas.iter().map(|meta| meta.id.to_string()).collect(),
        }
    }
}

pub async fn query(build: MetaBoardStruct) -> anyhow::Result<MetaV1Response> {
    let url = Url::from_str(&build.end_point.unwrap())?;
    let meta_board_id = build
        .meta_board_id
        .unwrap_or_else(|| Err(anyhow!("No meta-board-id provided")).unwrap());

    let variables = meta_board::Variables {
        metaboard: meta_board_id.into(),
    };

    let request_body = MetaBoard::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(url.clone()).json(&request_body).send().await?;
    let response_body: Response<meta_board::ResponseData> = res.json().await?;

    if let Some(meta_board) = response_body.data.and_then(|data| Some(data)) {
        let response: MetaV1Response = MetaV1Response::from(meta_board);
        Ok(response)
    } else {
        return Err(anyhow!("Failed to get metaboard"));
    }
}
