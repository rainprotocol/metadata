use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use ethabi::ethereum_types::{Address, H160, U256};
use graphql_client::{GraphQLQuery, Response};
use reqwest::Url;
use rust_bigint::BigInt;
use serde::{Serialize, Deserialize};
use serde_bytes::ByteBuf as Bytes;

use self::meta_board::ResponseData;

#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/subgraph/query/schema.json",
    query_path = "src/subgraph/query/metaboard/metaboard.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize"
)]

pub struct MetaBoard;

#[derive(Parser)]
pub struct MetaBoardStruct {
    // subgraph api endpoint. if not given, local graph-node endpoint is used
    #[arg(
        short = 'e',
        long = "end_point",
        default_value = "http://localhost:8000/subgraphs/name/test/test"
    )]
    end_point: Option<String>,
    // metaboard contracts address
    #[arg(short = 'm', long = "meta_board")]
    meta_board_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct SubgraphResponse {
    id: Address,
    address: Address,
    meta_count: U256,
    metas: Vec<String>,
}

impl SubgraphResponse {
    pub fn from(response: ResponseData) -> SubgraphResponse {
        let meta_board = response.meta_board.unwrap();
        let metas = meta_board.metas.unwrap();

        SubgraphResponse {
            id: H160::from_str(&String::from_utf8(meta_board.id.to_vec()).unwrap()).unwrap(),
            address: H160::from_str(&String::from_utf8(meta_board.address.to_vec()).unwrap())
                .unwrap(),
            meta_count: U256::from_dec_str(&meta_board.meta_count.to_str_radix(16)).unwrap(),
            metas: metas.iter().map(|meta| meta.id.to_string()).collect(),
        }
    }
}

pub async fn query(build: MetaBoardStruct) -> anyhow::Result<()> {
    let url = Url::from_str(&build.end_point.unwrap())?;
    let meta_board_id = build.meta_board_id.unwrap_or_else(|| Err(anyhow!("No meta-board-id provided")).unwrap());
    
    let variables = meta_board::Variables {
        metaboard: meta_board_id.into(),
    };

    let request_body = MetaBoard::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(url.clone()).json(&request_body).send().await?;
    let response_body: Response<meta_board::ResponseData> = res.json().await?;

    if let Some(meta_board) = response_body.data.and_then(|data| Some(data)) {
        let repsponse = SubgraphResponse::from(meta_board);

        serde_json::to_writer(std::io::stdout(),&repsponse)?;
    } else {
        tracing::warn!("Failed to get metaboard");
    }

    Ok(())
}
