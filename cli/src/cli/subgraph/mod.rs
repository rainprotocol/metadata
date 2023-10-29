use clap::{Subcommand, Parser};
use crate::subgraph::Subgraph;

#[derive(Subcommand, strum::Display)]
pub enum Sg {
    /// show all subgraph URLs
    All,
    /// show all native parser subgraph endpoint URLs
    NativeParser,
    /// show all legacy subgraph endpoint URLs
    Legacy,
    /// show subgraph endpoint URLs of specific chain
    Chain(Chain)
}

#[derive(Parser)]
pub struct Chain {
    /// the chain id of the network
    id: u64
}

pub fn dispatch(sg: Sg) -> anyhow::Result<()> {
    match sg {
        Sg::All => {
            for url in Subgraph::ALL.iter() {
                println!("{url}")
            }
        },
        Sg::NativeParser => {
            for url in Subgraph::NP.iter() {
                println!("{url}")
            }
        },
        Sg::Legacy => {
            for url in Subgraph::LEGACY.iter() {
                println!("{url}")
            }
        },
        Sg::Chain(chain_id) => {
            for url in Subgraph::of_chain(chain_id.id)?.iter() {
                println!("{url}")
            }
        },
    };
    Ok(())
}