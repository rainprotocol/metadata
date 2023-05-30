use schemars::schema_for;
use std::path::PathBuf;
use clap::Parser;
use strum::EnumString;

#[derive(Parser)]
pub struct Schema {
    /// # Schema
    /// One of a set of known JSON schemas that can be produced to match a subset
    /// of the validation performed on known metas. Additional validation beyond
    /// what can be expressed by JSON schema is performed when parsing and
    /// validating metadata.
    #[arg(value_parser = clap::value_parser!(KnownSchemas))]
    schema: KnownSchemas,
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    #[arg(short, long)]
    pretty_print: bool,
}

#[derive(Clone, EnumString)]
#[strum(serialize_all = "kebab_case")]
pub enum KnownSchemas {
    InterpreterCallerV1,
    OpV1,
}

pub async fn dispatch (schema: Schema) -> anyhow::Result<()> {
    let schema_json = match schema.schema {
        KnownSchemas::InterpreterCallerV1 => schema_for!(crate::meta::interpreter_caller::v1::InterpreterCallerMeta),
        KnownSchemas::OpV1 => schema_for!(crate::meta::op::v1::OpMeta),
    };

    let schema_string = if schema.pretty_print {
        serde_json::to_string_pretty(&schema_json)?
    } else {
        serde_json::to_string(&schema_json)?
    };

    if let Some(output_path) = schema.output_path {
        std::fs::write(output_path, schema_string)?;
    } else {
        println!("{}", schema_string);
    }

    Ok(())
}
