use std::sync::Arc;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use graphql_client::{GraphQLQuery, Response, QueryBody};
use super::types::authoring::v1::AuthoringMeta;

use super::{RainMetaDocumentV1Item, KnownMagic};

type Bytes = String;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/meta/query/schema.json",
    query_path = "src/meta/query/meta.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub(super) struct MetaQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/meta/query/schema.json",
    query_path = "src/meta/query/deployer.graphql",
    response_derives = "Debug, Serialize, Deserialize"
)]
pub(super) struct DeployerQuery;

/// response data struct for a meta
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MetaResponse {
    pub bytes: Vec<u8>,
}

/// response data struct for a deployer meta
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct DeployerMetaResponse {
    pub hash: String,
    pub bytes: Vec<u8>,
}

impl DeployerMetaResponse {
    /// get authoring meta bytes of this deployer meta
    pub fn get_authoring_meta(&self) -> Option<AuthoringMeta> {
        if let Ok(meta_maps) = RainMetaDocumentV1Item::cbor_decode(&self.bytes) {
            for meta_map in &meta_maps {
                if meta_map.magic == KnownMagic::AuthoringMetaV1 {
                    if let Ok(v) = meta_map.unpack() {
                        match AuthoringMeta::abi_decode_validate(&v) {
                            Ok(am) => return Some(am),
                            Err(_) => return None
                        }
                    }
                }
            }
            None
        } else {
            None
        }
    }
}

/// process a response for a meta
pub(super) async fn process_meta_query(
    client: Arc<Client>,
    request_body: &QueryBody<meta_query::Variables>,
    url: &str,
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
        .raw_bytes)
}

/// process a response for a deployer meta
pub(super) async fn process_deployer_query(
    client: Arc<Client>,
    request_body: &QueryBody<deployer_query::Variables>,
    url: &str,
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
        return Ok((
            res[0].constructor_meta_hash.clone(),
            res[0].constructor_meta.clone(),
        ));
    } else {
        return Err(anyhow::anyhow!("found no matching record!"));
    }
}
