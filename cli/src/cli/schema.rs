pub mod ls;
pub mod show;
pub mod validate;

use strum::{EnumString, EnumIter};
use clap::Subcommand;
use show::Show;
use validate::Validate;

#[derive(Subcommand)]
pub enum Schema {
    Ls,
    Show(Show),
    Validate(Validate),
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
        Schema::Show(s) => show::show(s)?,
        Schema::Validate(v) => validate::validate(v)?,
    })
}
