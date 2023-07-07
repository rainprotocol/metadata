use clap::Subcommand;

use crate::subgraph::deploy::Config;


#[derive(Subcommand)]
#[command(about = "Interact with an order(s) onchain and offchain.")]
pub enum Deploy {
    #[command(about = "deploy subgraph")]
    Deploy(Config)
}

pub async fn deploy(config: Config) -> anyhow::Result<()> {
    crate::subgraph::deploy::deploy(config).await?;
    Ok(())
}