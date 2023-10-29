pub mod meta;
pub(crate) mod cli;
pub(crate) mod solc;
pub(crate) mod subgraph;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::main()
}
