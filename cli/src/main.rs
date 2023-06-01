mod cli;
pub(crate) mod meta;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::main().await
}