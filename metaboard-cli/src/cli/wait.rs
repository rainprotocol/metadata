// use clap::Subcommand;

// #[derive(Subcommand)]
// #[command(about = "wait for subgraph to sync")]
// pub enum Wait {
//     #[command(about = "wait for subgraph to sync")]
//     Wait
// }

use crate::subgraph::wait::Wait;

pub async fn wait(wait: Wait) -> anyhow::Result<()> {
    crate::subgraph::wait::wait(wait).await?;
    Ok(())
}