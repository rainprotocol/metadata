use super::KnownMeta;
use super::types::op::v1::OpMeta;
use super::types::authoring::v1::AuthoringMeta;
use super::types::solidity_abi::v2::SolidityAbiMeta;
use super::types::interpreter_caller::v1::InterpreterCallerMeta;


fn normalize_json<'de, T: serde::Deserialize<'de> + serde::Serialize + validator::Validate>(data: &'de [u8]) -> anyhow::Result<Vec<u8>> {
    let parsed = serde_json::from_str::<T>(std::str::from_utf8(data)?)?;
    parsed.validate()?;
    Ok(serde_json::to_string(&parsed)?.as_bytes().to_vec())
}

impl KnownMeta {
    /// normalizes meta types and also performs validation on those that need validation
    pub fn normalize(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            KnownMeta::SolidityAbiV2 => normalize_json::<SolidityAbiMeta>(data)?,
            KnownMeta::InterpreterCallerMetaV1 => normalize_json::<InterpreterCallerMeta>(data)?,
            KnownMeta::OpV1 => normalize_json::<OpMeta>(data)?,
            KnownMeta::AuthoringMetaV1 => {
                // for AuthoringMeta since it can be a json or abi encoded bytes, we try to abi
                // decode first and then json deserialize if that fails, if either succeeds 
                // then the result of that will be abi encoded with validation
                match AuthoringMeta::abi_decode(&data.to_vec()) {
                    Ok(am) => am.abi_encode_validate()?,
                    _ => AuthoringMeta::abi_encode_validate(
                        &serde_json::from_str::<AuthoringMeta>(
                            std::str::from_utf8(data).or(Err(anyhow::anyhow!("deserialization attempts failed with both abi decoding and json deserialization")))?
                        ).or(Err(anyhow::anyhow!("deserialization attempts failed with both abi decoding and json deserialization")))?
                    )?
                }
            },
            _ => data.to_vec()
        })
    }
}