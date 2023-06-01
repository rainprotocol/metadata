pub mod ls;
pub mod show;

use strum::{EnumString, EnumIter};
use clap::Subcommand;
use show::Show;

#[derive(Subcommand)]
pub enum Schema {
    /// Print all known schemas.
    Ls,
    /// Print a given known schema.
    Show(Show),
}

#[derive(Clone, EnumString, EnumIter, strum::Display)]
#[strum(serialize_all = "kebab_case")]
pub enum KnownSchema {
    InterpreterCallerV1,
    OpV1,
}

pub fn dispatch (schema: Schema) -> anyhow::Result<()> {
    match schema {
        Schema::Ls => ls::ls(),
        Schema::Show(s) => show::show(s),
    }
}
