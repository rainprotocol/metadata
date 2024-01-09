pub mod ls;

use clap::Subcommand;

/// command related to rain magic numbers
#[derive(Subcommand)]
pub enum Magic {
    /// Print all known magic numbers.
    Ls,
}

pub fn dispatch(magic: Magic) -> anyhow::Result<()> {
    match magic {
        Magic::Ls => ls::ls(),
    }
}
