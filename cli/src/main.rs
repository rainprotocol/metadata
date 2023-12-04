pub mod meta;
pub(crate) mod cli;
pub(crate) mod solc;
pub(crate) mod subgraph;

#[cfg(target_feature = "cli")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::main()
}

#[cfg(not(target_feature = "cli"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    cli::main()
}
