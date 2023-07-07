use anyhow::anyhow;
use clap::Parser;
use std::{fs, process::Command};

#[derive(Parser)]
pub struct TestConfig {
    // script dir
    #[arg(short = 's', long = "script-dir")]
    script_dir: Option<String>,
}

pub async fn test(config: TestConfig) -> anyhow::Result<()> {
    let dir_path = config.script_dir.ok_or(anyhow!("No script dir provided"))?;

    let rpc_url = "http://127.0.0.1:8545";
    // Read the directory contents

    match fs::read_dir(dir_path) {
        Ok(entries) => {
            // Collect the file names that end with ".sg.sol"
            let mut files: Vec<_> = entries
                .filter_map(|entry| {
                    if let Ok(entry) = entry {
                        let path = entry.path();
                        if let Some(file_name) = path.file_name() {
                            if let Some(name) = file_name.to_str() {
                                if name.ends_with(".sg.sol") {
                                    return Some(name.to_owned());
                                }
                            }
                        }
                    }
                    None
                })
                .collect();

            // Sort the file names alphabetically
            files.sort();

            // Print the sorted file names
            for file in files {
                let output = Command::new("bash")
                    .args(&[
                        "-c",
                        &format!(
                            "forge script scripts/{} -vv --ffi --broadcast --rpc-url {}",
                            file, rpc_url
                        ),
                    ])
                    .output()?;

                if output.status.success() {
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    println!("{}", stdout);
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(anyhow::format_err!("Command failed with error: {}", stderr));
                }
            }
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
    Ok(())
}
