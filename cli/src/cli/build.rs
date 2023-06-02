use super::magic::KnownMagic;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
pub struct Build {
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    #[arg(short = 'M', long, default_value = "rain-meta-document-v1")]
    global_magic: KnownMagic,
    #[arg(short, long, num_args = 1..)]
    magic: Vec<KnownMagic>,
    #[arg(short, long, num_args = 1..)]
    input_path: Vec<PathBuf>,
}

fn build_bytes(b: &Build) -> anyhow::Result<Vec<u8>> {
    let mut bytess: Vec<Vec<u8>> = Vec::new();
    bytess.push(b.global_magic.to_prefix_bytes().to_vec());
    for (m, i) in b.magic.iter().zip(b.input_path.iter()) {
        bytess.push(m.to_prefix_bytes().to_vec());
        bytess.push(std::fs::read(i)?);
    }
    Ok(bytess.into_iter().flatten().collect())
}

pub fn build(b: Build) -> anyhow::Result<()> {
    crate::cli::output::output(&b.output_path, &build_bytes(&b)?)
}