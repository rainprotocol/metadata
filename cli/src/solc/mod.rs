use serde_json::Value;
use strum::EnumString;
use crate::error::Error;

/// Represent section of a solidity artifact to extract
#[derive(Copy, Clone, EnumString, strum::Display)]
#[strum(serialize_all = "kebab-case")]
pub enum ArtifactComponent {
    Abi,
    Bytecode,
    DeployedBytecode,
}

/// extracts the given section of a solidity artifact as [Value]
///
/// does not perform any checks on the returned [Value] such as if
/// it is null or not.
/// The given data should be utf8 encoded json string bytes
pub fn extract_artifact_component_json(
    component: ArtifactComponent,
    data: &[u8],
) -> Result<Value, Error> {
    let json = serde_json::from_str::<Value>(std::str::from_utf8(data)?)?;
    match component {
        ArtifactComponent::Abi => Ok(json["abi"].clone()),
        ArtifactComponent::Bytecode => Ok(json["bytecode"].clone()),
        ArtifactComponent::DeployedBytecode => Ok(json["deployedBytecode"].clone()),
    }
}
