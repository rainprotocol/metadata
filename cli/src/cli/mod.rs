use anyhow::Result;
use clap::command;
use clap::{Parser, Subcommand};
use crate::cli::schema::Schema;

mod schema;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    meta: Meta,
}

#[derive(Subcommand)]
pub enum Meta {
    #[command(subcommand)]
    Schema(Schema)
}

pub async fn dispatch(meta: Meta) -> Result<()> {
    match meta {
        Meta::Schema(schema) => schema::dispatch(schema).await,
    }
}

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    dispatch(cli.meta).await
}
