use anyhow::Result;
use clap::command;
use clap::{Parser, Subcommand};
use crate::subgraph::deploy::Config;
use crate::subgraph::wait::Wait;
mod query;
mod deploy;
mod wait;
mod test;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    metaboard: MetaBoardOption,
}

#[derive(Subcommand)]
pub enum MetaBoardOption {
    #[command(subcommand)]
    Query(query::Query),
    Deploy(Config),
    Wait(Wait),
    Test(test::TestConfig)
}

pub async fn dispatch(metaboard: MetaBoardOption) -> Result<()> {
    match metaboard {
        MetaBoardOption::Query(query) => query::dispatch(query).await,
        MetaBoardOption::Deploy(config) =>  deploy::deploy(config).await,
        MetaBoardOption::Wait(options) => wait::wait(options).await,
        MetaBoardOption::Test(config) => test::test(config).await
    }
}

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    dispatch(cli.metaboard).await
}
