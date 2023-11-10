use serde_json::Value;
use strum::{EnumIter, EnumString};

#[derive(Copy, Clone, EnumString, EnumIter, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum ArtifactComponent {
    Abi,
    Bytecode,
    DeployedBytecode,
}

pub fn extract_artifact_component_json(
    component: ArtifactComponent,
    data: &[u8],
) -> anyhow::Result<Value> {
    let json = serde_json::from_str::<Value>(std::str::from_utf8(data)?)?;
    match component {
        ArtifactComponent::Abi => Ok(json["abi"].clone()),
        ArtifactComponent::Bytecode => Ok(json["bytecode"].clone()),
        ArtifactComponent::DeployedBytecode => Ok(json["deployedBytecode"].clone()),
    }
}
