pub mod solc;
pub mod build;
pub mod magic;
pub mod schema;
pub mod output;
pub mod subgraph;
pub mod validate;

use clap::{Parser, Subcommand, command};


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    meta: Meta,
}

#[derive(Subcommand)]
pub enum Meta {
    #[command(subcommand)]
    Schema(schema::Schema),
    Validate(validate::Validate),
    #[command(subcommand)]
    Magic(magic::Magic),
    Build(build::Build),
    #[command(subcommand)]
    Solc(solc::Solc),
    #[command(subcommand)]
    Subgraph(subgraph::Sg)
}

pub fn dispatch(meta: Meta) -> anyhow::Result<()> {
    match meta {
        Meta::Build(build)          => build::build(build),
        Meta::Solc(solc)             => solc::dispatch(solc),
        Meta::Subgraph(sg)             => subgraph::dispatch(sg),
        Meta::Magic(magic)          => magic::dispatch(magic),
        Meta::Schema(schema)       => schema::dispatch(schema),
        Meta::Validate(validate) => validate::validate(validate),
    }
}

pub fn main() -> anyhow::Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    dispatch(cli.meta)
}
