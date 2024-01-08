use super::{
    KnownMeta,
    super::error::Error,
    types::{
        op::v1::OpMeta, authoring::v1::AuthoringMeta, solidity_abi::v2::SolidityAbiMeta,
        interpreter_caller::v1::InterpreterCallerMeta,
    },
};

fn normalize_json<'de, T>(data: &'de [u8]) -> Result<Vec<u8>, Error>
where
    T: serde::Deserialize<'de> + serde::Serialize + validator::Validate,
{
    let parsed = serde_json::from_str::<T>(std::str::from_utf8(data)?)?;
    parsed.validate()?;
    Ok(serde_json::to_string(&parsed)?.as_bytes().to_vec())
}

impl KnownMeta {
    /// normalizes meta types and also performs validation on those that need validation
    pub fn normalize(&self, data: &[u8]) -> Result<Vec<u8>, Error> {
        Ok(match self {
            KnownMeta::OpV1 => normalize_json::<OpMeta>(data)?,
            KnownMeta::SolidityAbiV2 => normalize_json::<SolidityAbiMeta>(data)?,
            KnownMeta::InterpreterCallerMetaV1 => normalize_json::<InterpreterCallerMeta>(data)?,
            KnownMeta::AuthoringMetaV1 => {
                // for AuthoringMeta since it can be a json or abi encoded bytes, we try to abi
                // decode first and then json deserialize if that fails, if either succeeds
                // then the result of that will be abi encoded with validation
                match AuthoringMeta::abi_decode(data) {
                    Ok(am) => am.abi_encode_validate()?,
                    _ => AuthoringMeta::abi_encode_validate(
                        &serde_json::from_str::<AuthoringMeta>(std::str::from_utf8(data)?)?,
                    )?,
                }
            }
            // rest of meta types are only pure bytes (ut8 strings or binary)
            // so no normalization/validation can happen for them at this level
            _ => data.to_vec(),
        })
    }
}
