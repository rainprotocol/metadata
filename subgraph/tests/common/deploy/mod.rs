use anyhow::anyhow;
use mustache::MapBuilder;
use std::{fs, path::PathBuf, process::Command};

pub struct Config {
    // name of subgraph user/subgraph-endpoint
    subgraph_name: Option<String>,
    // subgraph version lable
    version_lable: Option<String>,
    // blockchian network
    network: Option<String>,
    // subgraph template of subgraph.yaml
    subgraph_template: Option<PathBuf>,
    // contracts address
    contract_address: Option<String>,
    // block-number
    block_number: Option<String>,
    // graph access token
    graph_access_token: Option<String>,
    // endpoint
    end_point: Option<String>,
    // output path
    output_path: Option<PathBuf>,
    // Root dir
    root_dir: Option<PathBuf>,
}

pub async fn deploy(config: Config) -> anyhow::Result<()> {
    let network = config
        .network
        .unwrap_or_else(|| Err(anyhow!("No network provided")).unwrap());

    let contract = config
        .contract_address
        .unwrap_or_else(|| Err(anyhow!("No contract address provided")).unwrap());

    let block_number = config
        .block_number
        .unwrap_or_else(|| Err(anyhow!("No block-number provided")).unwrap());

    let output_path = config
        .output_path
        .unwrap_or_else(|| Err(anyhow!("No output path provided")).unwrap());

    let subgraph_template = config
        .subgraph_template
        .unwrap_or_else(|| Err(anyhow!("No subgraph-template path provided")).unwrap());

    let root_dir = config
        .root_dir
        .unwrap_or_else(|| Err(anyhow!("No root path provided")).unwrap());

    let end_point = config
        .end_point
        .unwrap_or_else(|| Err(anyhow!("No end-point provided")).unwrap());

    let subgraph_name = config
        .subgraph_name
        .unwrap_or_else(|| Err(anyhow!("No subgraph-name provided provided")).unwrap());

    let version_lable = config
        .version_lable
        .unwrap_or_else(|| Err(anyhow!("No version-lable provided provided")).unwrap());

    if network != "localhost" {
        let graph_access_token = config
            .graph_access_token
            .unwrap_or_else(|| Err(anyhow!("Graph Access Token is not proiveded.")).unwrap());

        let output = Command::new("bash")
            .current_dir(format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                root_dir.to_str().unwrap()
            ))
            .args(&[
                "-c",
                &format!(
                    "npx graph auth --product hosted-service {}",
                    graph_access_token
                ),
            ])
            .output()
            .expect("Failed graph auth command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("{}", stderr));
        }
    }

    let data = MapBuilder::new()
        .insert_str("network", &network)
        .insert_str("MetaBoard", contract)
        .insert_str("MetaBoardBlock", block_number)
        .build();

    let template = fs::read_to_string(subgraph_template.clone()).expect(&format!(
        "Fail to read {}",
        subgraph_template.to_str().unwrap()
    ));

    let renderd = mustache::compile_str(&template)
        .expect("Failed to compile template")
        .render_data_to_string(&data)
        .expect("Failed to render template");

    let _write = fs::write(output_path, renderd)?;

    let output = Command::new("bash")
        .current_dir(format!(
            "{}/{}",
            std::env::current_dir().unwrap().display(),
            root_dir.to_str().unwrap()
        ))
        .args(&["-c", "npx graph codegen && npx graph build"])
        .output()
        .expect("Failed graph codegen and graph build command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("{}", stderr));
    }

    if network != "localhost" {
        let output = Command::new("bash")
            .current_dir(format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                root_dir.to_str().unwrap()
            ))
            .args(&[
                "-c",
                &format!("npx graph deploy {} {}", end_point, subgraph_name),
            ])
            .output()
            .expect("Failed graph deploy command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("{}", stderr));
        }
    } else {
        let _output = Command::new("bash")
            .current_dir(format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                root_dir.to_str().unwrap()
            ))
            .args(&[
                "-c",
                &format!("npx graph create --node {} {}", end_point, subgraph_name),
            ])
            .output()
            .expect("Failed graph create command");

        let output = Command::new("bash")
            .current_dir(format!(
                "{}/{}",
                std::env::current_dir().unwrap().display(),
                root_dir.to_str().unwrap()
            ))
            .args(&[
                "-c",
                &format!(
                    "npx graph deploy --node {} --ipfs http://localhost:5001 {}  --version-label {}",
                    end_point, subgraph_name, version_lable
                ),
            ])
            .output()
            .expect("Failed local deploy command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("{}", stderr));
        }
    }

    Ok(())
}
