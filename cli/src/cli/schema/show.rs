use std::path::PathBuf;
use super::KnownSchema;
use clap::Parser;
use schemars::schema_for;

#[derive(Parser)]
pub struct Show {
    /// One of a set of known JSON schemas that can be produced to match a subset
    /// of the validation performed on known metas. Additional validation beyond
    /// what can be expressed by JSON schema is performed when parsing and
    /// validating metadata.
    #[arg(value_parser = clap::value_parser!(KnownSchema))]
    schema: KnownSchema,
    /// If provided the schema will be written to the given path instead of
    /// stdin.
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    /// If true the schema will be pretty printed. Defaults to false.
    #[arg(short, long)]
    pretty_print: bool,
}

pub fn show(s: Show) -> anyhow::Result<()> {
    let schema_json = match s.schema {
        KnownSchema::InterpreterCallerV1 => schema_for!(crate::meta::interpreter_caller::v1::InterpreterCallerMeta),
        KnownSchema::OpV1 => schema_for!(crate::meta::op::v1::OpMeta),
    };

    let schema_string = if s.pretty_print {
        serde_json::to_string_pretty(&schema_json)?
    } else {
        serde_json::to_string(&schema_json)?
    };

    if let Some(output_path) = s.output_path {
        std::fs::write(output_path, schema_string)?;
    } else {
        println!("{}", schema_string);
    }
    Ok(())
}