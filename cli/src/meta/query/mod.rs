use std::sync::Arc;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use graphql_client::{GraphQLQuery, Response, QueryBody};

// type ID = String;
type Bytes = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/meta/query/schema.json",
    query_path = "src/meta/query/meta.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)] 
pub(super) struct MetaQuery;

/// response data struct for a meta
#[derive(Serialize, Deserialize, Debug)]
pub struct MetaResponse {
    pub bytes: Vec<u8>
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/meta/query/schema.json",
    query_path = "src/meta/query/deployer.graphql",
    response_derives = "Debug, Serialize, Deserialize",
)]
pub(super) struct DeployerQuery;

/// response data struct for a deployer meta
#[derive(Serialize, Deserialize, Debug)]
pub struct DeployerMetaResponse {
    pub hash: String,
    pub bytes: Vec<u8>
}

/// process a response for a meta
pub(super) async fn process_meta_query(
    client: Arc<Client>, 
    request_body: &QueryBody<meta_query::Variables>, 
    url: &String
) -> anyhow::Result<String> {
    Ok(client
        .post(Url::parse(url)?)
        .json(request_body)
        .send()
        .await?
        .json::<Response<meta_query::ResponseData>>()
        .await?
        .data
        .ok_or(anyhow::anyhow!("found no matching record!"))?
        .meta
        .ok_or(anyhow::anyhow!("found no matching record!"))?
        .raw_bytes
    )
}

/// process a response for a deployer meta
pub(super) async fn process_deployer_query(
    client: Arc<Client>, 
    request_body: &QueryBody<deployer_query::Variables>, 
    url: &String
) -> anyhow::Result<(String, String)> {
    let res = client
        .post(Url::parse(url)?)
        .json(request_body)
        .send()
        .await?
        .json::<Response<deployer_query::ResponseData>>()
        .await?
        .data
        .ok_or(anyhow::anyhow!("found no matching record!"))?
        .expression_deployers;

    if res.len() > 0 {
        return Ok((res[0].constructor_meta_hash.clone(), res[0].constructor_meta.clone()));
    } else {
        return Err(anyhow::anyhow!("found no matching record!"))
    }
}
