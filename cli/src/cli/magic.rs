pub mod ls;

use clap::Subcommand;
use strum::{EnumString, EnumIter};


#[derive(Subcommand)]
pub enum Magic {
    /// Print all known magic numbers.
    Ls,
}

pub fn dispatch (magic: Magic) -> anyhow::Result<()> {
    match magic {
        Magic::Ls => ls::ls(),
    }
}