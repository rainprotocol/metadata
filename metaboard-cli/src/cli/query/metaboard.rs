use crate::subgraph::query::metaboard::{MetaBoardStruct, query};

pub async fn meta_board(build: MetaBoardStruct) -> anyhow::Result<()> {
    query(build).await
}