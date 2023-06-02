use std::io::Write;
use std::path::PathBuf;

pub fn output(output_path: Option<PathBuf>, bytes: &[u8]) -> anyhow::Result<()> {
    Ok(if let Some(output_path) = output_path {
        std::fs::write(output_path, bytes)?
    } else {
        std::io::stdout().write_all(bytes)?
    })
}