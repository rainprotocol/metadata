use anyhow::Result;
use clap::command;
use clap::{Parser, Subcommand};

pub mod schema;
pub mod validate;
pub mod magic;

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
}

pub fn dispatch(meta: Meta) -> Result<()> {
    match meta {
        Meta::Schema(schema) => schema::dispatch(schema),
        Meta::Validate(validate) => validate::validate(validate),
        Meta::Magic(magic) => magic::dispatch(magic),
    }
}

pub fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    dispatch(cli.meta)
}
