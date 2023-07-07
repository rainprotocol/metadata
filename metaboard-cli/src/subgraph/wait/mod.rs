use anyhow::anyhow;
use ethabi::ethereum_types::U256;
use graphql_client::{GraphQLQuery, Response};
use reqwest::Url;
use rust_bigint::BigInt;
use std::{
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use serde::{Serialize};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/subgraph/wait/schema.json",
    query_path = "src/subgraph/wait/query.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize"
)]

pub struct SyncStatus;

use clap::Parser;

use self::sync_status::Health;

#[derive(Parser)]
pub struct Wait {
    // block number to wait for
    #[arg(short = 'b', long = "block-number")]
    block_number: Option<u64>,
}

#[derive(Serialize)]
struct SynceResponse {
    status: U256
}

pub async fn wait(option: Wait) -> anyhow::Result<()> {
    let block_number = option
        .block_number
        .unwrap_or_else(|| Err(anyhow!("No block-number provided")).unwrap());

    let url = Url::from_str("http://localhost:8030/graphql")?;

    let variables = sync_status::Variables {};

    let request_body = SyncStatus::build_query(variables);

    let clint = reqwest::Client::new();
    let deadline = SystemTime::now().duration_since(UNIX_EPOCH)? + Duration::from_secs(5);

    loop {
        if deadline < SystemTime::now().duration_since(UNIX_EPOCH)? {
            let response = SynceResponse{
                status: U256::from_str("0")?
            };
            serde_json::to_writer(std::io::stdout(), &response)?;
            break Ok(());
        }

        let response = clint.post(url.clone()).json(&request_body).send().await?;
        let response_body: Response<sync_status::ResponseData> = response.json().await?;

        if let Some(data) = response_body.data.and_then(|data| Some(data)) {
            let sync_data = &data.indexing_status_for_current_version.unwrap();
            // if(sync_data.synced )
            let chain = &sync_data.chains[0];

            let block = &chain.latest_block.as_ref().unwrap().number;

            let block = block.to_str_radix(10).parse::<u64>()?;
            // println!("{}", block);
            // println!("{}", sync_data.synced);
            let health = &sync_data.health;

            if sync_data.synced && block >= block_number {
                let response = SynceResponse{
                    status: U256::from_str("1")?
                };
                serde_json::to_writer(std::io::stdout(), &response)?;
                break Ok(());
            }

            if let Health::failed = health {
                let response = SynceResponse{
                    status: U256::from_str("0")?
                };
                serde_json::to_writer(std::io::stdout(), &response)?;
                break Ok(());
            }
        }
    }
}
