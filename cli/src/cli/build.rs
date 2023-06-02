use super::magic::KnownMagic;
use std::path::PathBuf;
use clap::Parser;


#[derive(Parser)]
pub struct Build {
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    #[arg(short, long, num_args = 1..)]
    magic: Vec<KnownMagic>,
    #[arg(short, long, num_args = 1..)]
    input_path: Vec<PathBuf>,
}

pub fn build(b: Build) -> anyhow::Result<()> {

    let mut bytess: Vec<Vec<u8>> = Vec::new();
    for (m, i) in b.magic.into_iter().zip(b.input_path.into_iter()) {
        bytess.push((m as u64).to_le_bytes().to_vec());
        bytess.push(std::fs::read(i)?);
    }
    crate::cli::output::output(b.output_path, &bytess.into_iter().flatten().collect::<Vec<u8>>())
}