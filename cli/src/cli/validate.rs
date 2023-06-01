use super::schema::KnownSchema;
use clap::Parser;
use std::path::PathBuf;
use crate::meta::interpreter_caller::v1::InterpreterCallerMeta;
use crate::meta::op::v1::OpMeta;

#[derive(Parser)]
pub struct Validate {
    /// One of a set of known JSON schemas that can be produced to match a subset
    /// of the validation performed on known metas. Additional validation beyond
    /// what can be expressed by JSON schema is performed when parsing and
    /// validating metadata.
    #[arg(short, long)]
    schema: KnownSchema,
    /// The input path to the json serialized metadata to validate against the
    /// known schema.
    #[arg(short, long)]
    input_path: PathBuf,
}

pub fn validate(v: Validate) -> anyhow::Result<()> {
    let data: String = std::fs::read_to_string(v.input_path)?;

    Ok(match v.schema {
        KnownSchema::InterpreterCallerV1 => validator::Validate::validate(&serde_json::from_str::<InterpreterCallerMeta>(&data)?)?,
        KnownSchema::OpV1 => validator::Validate::validate(&serde_json::from_str::<OpMeta>(&data)?)?,
    })
}