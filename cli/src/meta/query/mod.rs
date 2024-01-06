use std::sync::Arc;
use reqwest::Client;
use alloy_primitives::hex::decode;
use serde::{Deserialize, Serialize};
use graphql_client::{GraphQLQuery, Response, QueryBody};
use super::{
    RainMetaDocumentV1Item, KnownMagic, types::authoring::v1::AuthoringMeta, super::error::Error,
};

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
    #[serde(with = "serde_bytes")]
    pub tx_hash: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub bytecode_meta_hash: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub meta_hash: Vec<u8>,
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
) -> Result<MetaResponse, Error> {
    Ok(MetaResponse {
        bytes: decode(
            client
                .post(url)
                .json(request_body)
                .send()
                .await
                .map_err(Error::ReqwestError)?
                .json::<Response<meta_query::ResponseData>>()
                .await
                .map_err(Error::ReqwestError)?
                .data
                .ok_or(Error::NoRecordFound)?
                .meta
                .ok_or(Error::NoRecordFound)?
                .raw_bytes,
        )
        .or(Err(Error::NoRecordFound))?,
    })
}

/// process a response for a deployer by resolving if a record was found or reject if nothing found or rejected with error
/// This is because graphql responses are not rejected even if there was no record found for the request
pub(super) async fn process_deployer_query(
    client: Arc<Client>,
    request_body: &QueryBody<deployer_query::Variables>,
    url: &str,
) -> Result<DeployerResponse, Error> {
    let res = client
        .post(url)
        .json(request_body)
        .send()
        .await
        .map_err(Error::ReqwestError)?
        .json::<Response<deployer_query::ResponseData>>()
        .await
        .map_err(Error::ReqwestError)?
        .data
        .ok_or(Error::NoRecordFound)?
        .expression_deployers;

    if !res.is_empty() {
        let bytecode = if let Some(v) = &res[0].bytecode {
            decode(v).or(Err(Error::NoRecordFound))?
        } else {
            return Err(Error::NoRecordFound);
        };
        let parser = if let Some(v) = &res[0].parser {
            decode(&v.parser.deployed_bytecode).or(Err(Error::NoRecordFound))?
        } else {
            return Err(Error::NoRecordFound);
        };
        let store = if let Some(v) = &res[0].store {
            decode(&v.store.deployed_bytecode).or(Err(Error::NoRecordFound))?
        } else {
            return Err(Error::NoRecordFound);
        };
        let interpreter = if let Some(v) = &res[0].interpreter {
            decode(&v.interpreter.deployed_bytecode).or(Err(Error::NoRecordFound))?
        } else {
            return Err(Error::NoRecordFound);
        };
        let bytecode_meta_hash = if res[0].meta.len() == 1 {
            decode(&res[0].meta[0].id).or(Err(Error::NoRecordFound))?
        } else {
            return Err(Error::NoRecordFound);
        };
        let tx_hash = if let Some(v) = &res[0].deploy_transaction {
            decode(&v.id).or(Err(Error::NoRecordFound))?
        } else {
            return Err(Error::NoRecordFound);
        };
        let meta_hash = decode(&res[0].constructor_meta_hash).or(Err(Error::NoRecordFound))?;
        let meta_bytes = decode(&res[0].constructor_meta).or(Err(Error::NoRecordFound))?;
        Ok(DeployerResponse {
            meta_hash,
            meta_bytes,
            bytecode,
            parser,
            store,
            interpreter,
            bytecode_meta_hash,
            tx_hash,
        })
    } else {
        Err(Error::NoRecordFound)
    }
}
