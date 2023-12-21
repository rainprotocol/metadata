use std::sync::Arc;
use reqwest::{Client, Url};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use super::{RainMetaDocumentV1Item, KnownMagic, types::authoring::v1::AuthoringMeta};

/// response data struct for a meta
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MetaResponse {
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,
}

/// response data struct for a deployer meta
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeployerNPResponse {
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

impl DeployerNPResponse {
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

/// get deployer query string
pub fn get_deployer_query(hash: &str) -> String {
    format!(
        r#"query DeployerQuery {{ expressionDeployers( first: 1
    where: {{ or: [
        {{ deployTransaction_: {{ id: "{}" }} }}, 
        {{ meta_: {{ id: "{}" }} }}
    ] }}
) {{ 
    constructorMetaHash 
    constructorMeta
    deployTransaction {{
        id
    }}
    bytecode
    parser {{
        parser {{
            deployedBytecode
        }}
    }}
    store {{
        store {{
            deployedBytecode
        }}
    }}
    interpreter {{
        interpreter {{
            deployedBytecode
        }}
    }}
    meta(first: 1 where: {{ magicNumber: "18436497220406627634" }}) {{
        id
    }}
}} }}"#,
        hash, hash
    )
}

/// get meta query string
pub fn get_meta_query(hash: &str) -> String {
    format!(
        r#"query MetaQuery {{ meta( id: "{}" ) {{ rawBytes }} }}"#,
        hash
    )
}

/// process a response for a meta
pub(super) async fn process_meta_query(
    client: Arc<Client>,
    query_string: &str,
    url: &str,
) -> anyhow::Result<MetaResponse> {
    let mut query = HashMap::new();
    query.insert("query", query_string);
    let result = &client
        .post(Url::parse(url)?)
        .json(&query)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?["data"]["meta"]["rawBytes"];

    if result.is_string() {
        Ok(MetaResponse {
            bytes: alloy_primitives::hex::decode(result.as_str().unwrap())
            .or(Err(anyhow::anyhow!("found no matching record!")))?
        })
    } else {
        Err(anyhow::anyhow!("found no record"))
    }
}

/// process a response for a deployer meta
pub(super) async fn process_deployer_query(
    client: Arc<Client>,
    query_string: &str,
    url: &str,
) -> anyhow::Result<DeployerNPResponse> {
    let mut query = HashMap::new();
    query.insert("query", query_string);
    let res = &client
        .post(Url::parse(url)?)
        .json(&query)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?["data"]["expressionDeployers"][0];

    if res.is_object() && !res.is_null() {
        let bytecode = if let Some(v) = res["bytecode"].as_str() {
            alloy_primitives::hex::decode(v)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let parser = if let Some(v) = res["parser"]["parser"]["deployedBytecode"].as_str() {
            alloy_primitives::hex::decode(v)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let store = if let Some(v) = res["store"]["store"]["deployedBytecode"].as_str() {
            alloy_primitives::hex::decode(v)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let interpreter =
            if let Some(v) = res["interpreter"]["interpreter"]["deployedBytecode"].as_str() {
                alloy_primitives::hex::decode(v)
                    .or(Err(anyhow::anyhow!("found no matching record!")))?
            } else {
                return Err(anyhow::anyhow!("found no matching record!"));
            };
        let bytecode_meta_hash = if let Some(v) = res["meta"][0]["id"].as_str() {
            v.to_ascii_lowercase()
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let tx_hash = if let Some(v) = res["deployTransaction"]["id"].as_str() {
            v.to_ascii_lowercase()
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let meta_hash = if let Some(v) = res["constructorMetaHash"].as_str() {
            v.to_ascii_lowercase()
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        let meta_bytes = if let Some(v) = res["constructorMeta"].as_str() {
            alloy_primitives::hex::decode(v)
                .or(Err(anyhow::anyhow!("found no matching record!")))?
        } else {
            return Err(anyhow::anyhow!("found no matching record!"));
        };
        Ok(DeployerNPResponse {
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
        return Err(anyhow::anyhow!("found no matching record!"));
    }
}
