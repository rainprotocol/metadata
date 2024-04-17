use crate::cynic_client::{CynicClient, CynicClientError};
use crate::types::metas::*;
use alloy_primitives::hex::encode;
use reqwest::Url;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaboardSubgraphClientError {
    #[error(transparent)]
    CynicClientError(#[from] CynicClientError),
    #[error("Subgraph query returned no data")]
    Empty,
}

pub struct MetaboardSubgraphClient {
    url: Url,
}

impl CynicClient for MetaboardSubgraphClient {
    fn get_base_url(&self) -> Url {
        self.url.clone()
    }
}

impl MetaboardSubgraphClient {
    pub fn new(url: Url) -> Self {
        Self { url }
    }

    /// Find all metas with a given hash
    pub async fn get_meta_by_hash(
        &self,
        metahash: &[u8; 32],
    ) -> Result<Vec<MetaV1>, MetaboardSubgraphClientError> {
        let hex_string = encode(metahash);
        let metahash = format!("0x{}", hex_string);
        let data = self
            .query::<MetasByHash, MetasByHashVariables>(MetasByHashVariables {
                metahash: Some(Bytes(metahash)),
            })
            .await?;

        Ok(data.meta_v1_s)
    }
}
