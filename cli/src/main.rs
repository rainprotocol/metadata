mod cli;
pub mod meta;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    cli::main().await
}