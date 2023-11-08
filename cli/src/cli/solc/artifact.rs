use clap::Parser;
use std::path::PathBuf;
use crate::solc::ArtifactComponent;
use crate::cli::output::SupportedOutputEncoding;

#[derive(Parser)]
pub struct Artifact {
    /// artifact component: abi, bytecode, deployed-bytecode
    #[arg(value_parser = clap::value_parser!(ArtifactComponent), short, long)]
    component: ArtifactComponent,
    /// input path of the artifact file
    #[arg(short, long)]
    input_path: PathBuf,
    /// If provided the extracted artifact component will be written to the given
    /// path intead of stdout.
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    /// If true the extracted component will be pretty printed. Defaults to false.
    #[arg(short, long)]
    pretty_print: bool,
    #[arg(short = 'E', long, default_value = "binary")]
    output_encoding: SupportedOutputEncoding,
}

pub fn artifact(artifact: Artifact) -> anyhow::Result<()> {
    let extracted_component = crate::solc::extract_artifact_component_json(
        artifact.component,
        &std::fs::read(artifact.input_path)?,
    )?;

    let component_string = if artifact.pretty_print {
        serde_json::to_string_pretty(&extracted_component)?
    } else {
        serde_json::to_string(&extracted_component)?
    };

    crate::cli::output::output(
        &artifact.output_path,
        artifact.output_encoding,
        component_string.as_bytes(),
    )
}
