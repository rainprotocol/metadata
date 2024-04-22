use crate::cynic_client::{CynicClient, CynicClientError};
use crate::types::metas::*;
use alloy_primitives::hex::{decode, encode, FromHexError};
use reqwest::Url;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MetaboardSubgraphClientError {
    #[error(transparent)]
    CynicClientError(#[from] CynicClientError),
    #[error("Subgraph query returned no data")]
    Empty,
    #[error(transparent)]
    FromHexError(#[from] FromHexError),
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
    pub async fn get_metabytes_by_hash(
        &self,
        metahash: &[u8; 32],
    ) -> Result<Vec<Vec<u8>>, MetaboardSubgraphClientError> {
        let hex_string = encode(metahash);
        let metahash = format!("0x{}", hex_string);

        let data = self
            .query::<MetasByHash, MetasByHashVariables>(MetasByHashVariables {
                metahash: Some(Bytes(metahash)),
            })
            .await?;

        if data.meta_v1_s.is_empty() {
            return Err(MetaboardSubgraphClientError::Empty);
        }

        // decode all the metas
        let mut meta_bytes = Vec::new();
        for meta in data.meta_v1_s {
            meta_bytes.push(decode(&meta.meta.0)?);
        }

        Ok(meta_bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::hex::encode;
    use httpmock::Method::POST;
    use httpmock::MockServer;
    use reqwest::Url;

    #[tokio::test]
    async fn test_get_metabytes_by_hash_success() {
        let server = MockServer::start_async().await;
        let url = Url::parse(&server.url("/")).unwrap();

        let hash = [1u8; 32];

        // Mock a successful response
        server.mock(|when, then| {
            when.method(POST).path("/").body_contains(encode(hash)); // You need to tailor this to the actual body sent
            then.status(200).json_body_obj(&{
                serde_json::json!({
                    "data": {
                        "metaV1S": [
                            {
                             "meta": "0x01",
                             "metaHash": "0x00",
                             "sender": "0x00",
                             "id": "0x00",
                             "metaBoard": {
                                 "id": "0x00",
                                 "metas": [],
                                 "address": "0x00",
                             },
                             "subject": "0x00",
                            },
                            {
                                "meta": "0x02",
                                "metaHash": "0x00",
                                "sender": "0x00",
                                "id": "0x00",
                                "metaBoard": {
                                    "id": "0x00",
                                    "metas": [],
                                    "address": "0x00",
                                },
                                "subject": "0x00",
                               }
                        ]
                    }
                })
            });
        });

        let client = MetaboardSubgraphClient::new(url);

        let result = client.get_metabytes_by_hash(&hash).await;

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec![1]);
        assert_eq!(result[1], vec![2]);
    }

    #[tokio::test]
    async fn test_get_metabytes_by_hash_empty() {
        let server = MockServer::start_async().await;
        let url = Url::parse(&server.url("/")).unwrap();

        // Mock an empty response
        server.mock(|when, then| {
            when.method(POST).path("/").body_contains("metahash");
            then.status(200).json_body_obj(&{
                serde_json::json!({
                    "data": {
                        "metaV1S": []
                    }
                })
            });
        });

        let client = MetaboardSubgraphClient::new(url);
        let hash = [0u8; 32];

        let result = client.get_metabytes_by_hash(&hash).await;

        assert!(result.is_err());
        match result {
            Err(MetaboardSubgraphClientError::Empty) => (),
            _ => panic!("Unexpected result: {:?}", result),
        }
    }
}
