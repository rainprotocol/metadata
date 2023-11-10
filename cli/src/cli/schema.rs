pub mod ls;
pub mod show;

use clap::Subcommand;
use show::Show;

/// command related to meta json schema
#[derive(Subcommand)]
pub enum Schema {
    /// Print all known schemas.
    Ls,
    /// Print a given known schema.
    Show(Show),
}

pub fn dispatch(schema: Schema) -> anyhow::Result<()> {
    match schema {
        Schema::Ls => ls::ls(),
        Schema::Show(s) => show::show(s),
    }
}
