pub mod ls;
pub mod show;

use strum::{EnumString, EnumIter};
use clap::Subcommand;
use show::Show;

#[derive(Subcommand)]
pub enum Schema {
    Ls,
    Show(Show),
}

#[derive(Clone, EnumString, EnumIter, strum::Display)]
#[strum(serialize_all = "kebab_case")]
pub enum KnownSchema {
    InterpreterCallerV1,
    OpV1,
}

pub async fn dispatch (schema: Schema) -> anyhow::Result<()> {
    Ok(match schema {
        Schema::Ls => ls::ls(),
        Schema::Show(schema) => show::show(schema)?,
    })
}
