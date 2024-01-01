pub mod meta;
pub mod error;
pub(crate) mod cli;
pub(crate) mod solc;
pub(crate) mod subgraph;

#[cfg(feature = "tokio-full")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::main()
}

#[cfg(not(feature = "tokio-full"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    cli::main()
}
