use clap::Subcommand;

use crate::subgraph::query::{metaboard::MetaBoardStruct, metav1::MetaV1Struct};

pub mod metaboard;
pub mod metav1;
#[derive(Subcommand)]
#[command(about = "Interact with an order(s) onchain and offchain.")]
pub enum Query {
    #[command(about = "show metaboard entity")]
    MetaBoard(MetaBoardStruct),
    #[command(about = "show MetaV1 entity")]
    MetaV1(MetaV1Struct)
}

pub async fn dispatch(meta_board: Query) -> anyhow::Result<()> {
    match  meta_board {
        Query::MetaBoard(build) => metaboard::meta_board(build).await,
        Query::MetaV1(build) => metav1::meta_v1(build).await
    }
}