use std::sync::Arc;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use graphql_client::{GraphQLQuery, Response, QueryBody};
use super::{RainMetaDocumentV1Item, KnownMagic, types::authoring::v1::AuthoringMeta};

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
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
}

/// response data struct for an ExpressionDeployer
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeployerResponse {
    pub tx_hash: String,
    pub bytecode_meta_hash: String,
    pub meta_hash: String,
    #[serde(with = "serde_bytes")]
    pub meta_bytes: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub bytecode: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub parser: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub store: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub interpreter: Vec<u8>,
}

impl DeployerResponse {
    /// get authoring meta bytes of this deployer meta
    pub fn get_authoring_meta(&self) -> Option<AuthoringMeta> {
        if let Ok(meta_maps) = RainMetaDocumentV1Item::cbor_decode(&self.meta_bytes) {
            for meta_map in &meta_maps {
                if meta_map.magic == KnownMagic::AuthoringMetaV1 {
                    if let Ok(v) = meta_map.unpack() {
                        match AuthoringMeta::abi_decode_validate(&v) {
                            Ok(am) => return Some(am),
                            Err(_) => return None,
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

/// Process a response for a meta by resolving if a record was found or reject if nothing found or rejected with error
/// This is because graphql responses are not rejected even if there was no record found for the request
pub(super) async fn process_meta_query(
    client: Arc<Client>,
    request_body: &QueryBody<meta_query::Variables>,
    url: &str,
) -> anyhow::Result<MetaResponse> {
    Ok(MetaResponse {
        bytes: alloy_primitives::hex::decode(
            client
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
                .raw_bytes,
        )
        .or(Err(anyhow::anyhow!("found no matching record!")))?,
    })
}

/// process a response for a deployer by resolving if a record was found or reject if nothing found or rejected with error
/// This is because graphql responses are not rejected even if there was no record found for the request
pub(super) async fn process_deployer_query(
    client: Arc<Client>,
    request_body: &QueryBody<deployer_query::Variables>,
    url: &str,
) -> anyhow::Result<DeployerResponse> {
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
        let bytecode = if let Some(v) = &res[0].bytecode {
            alloy_primitives::hex::decode(v)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let parser = if let Some(v) = &res[0].parser {
            alloy_primitives::hex::decode(&v.parser.deployed_bytecode)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let store = if let Some(v) = &res[0].store {
            alloy_primitives::hex::decode(&v.store.deployed_bytecode)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let interpreter = if let Some(v) = &res[0].interpreter {
            alloy_primitives::hex::decode(&v.interpreter.deployed_bytecode)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let bytecode_meta_hash = if res[0].meta.len() == 1 {
            res[0].meta[0].id.to_ascii_lowercase()
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let tx_hash = if let Some(v) = &res[0].deploy_transaction {
            v.id.to_ascii_lowercase()
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let meta_hash = res[0].constructor_meta_hash.to_ascii_lowercase();
        let meta_bytes = alloy_primitives::hex::decode(&res[0].constructor_meta)
            .or(Err(anyhow::anyhow!("found no matching record!")))?;
        return Ok(DeployerResponse {
            meta_hash,
            meta_bytes,
            bytecode,
            parser,
            store,
            interpreter,
            bytecode_meta_hash,
            tx_hash,
        });
    } else {
        return Err(anyhow::anyhow!("found no matching record!"));
    }
}
