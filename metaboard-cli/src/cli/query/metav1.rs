use crate::subgraph::query::metav1::{MetaV1Struct, query};

pub async fn meta_v1(build: MetaV1Struct) -> anyhow::Result<()> {
    query(build).await
}