use clap::{Subcommand};
use schemars::schema_for;

#[derive(Subcommand)]
#[command(about = "Interact with meta schemas.")]
pub enum Schema {
    #[command(about = "Show the schema for op meta.")]
    OpV1
}

pub async fn dispatch (schema: Schema) -> anyhow::Result<()> {
    Ok(println!("{}", serde_json::to_string(&match schema {
        Schema::OpV1 => schema_for!(crate::meta::op::v1::OpMeta),
    })?))
}
