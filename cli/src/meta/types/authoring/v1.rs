use serde::Serialize;
use serde::Deserialize;
use validator::Validate;
use schemars::JsonSchema;
use validator::ValidationErrors;
use alloy_sol_types::{SolType, sol};

use super::super::{
    common::v1::REGEX_RAIN_SYMBOL,
    common::v1::REGEX_RAIN_STRING,
    super::{
        MetaMap, 
        super::utils::{str_to_bytes32, bytes32_to_str}
    }
};


/// authoring meta struct
pub type AuthoringMetaStruct = sol!((bytes32, uint8, string));

/// array of authoring meta struct
pub type AuthoringMetaStructArray = sol!((bytes32, uint8, string)[]);

/// # Authoring Meta
/// array of native parser opcode metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct AuthoringMeta(pub Vec<AuthoringMetaItem>);

/// AuthoringMeta single item
#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringMetaItem {
    /// # Word
    /// Primary word used to identify the opcode.
    #[validate(regex(path = "REGEX_RAIN_SYMBOL", message = "Must be alphanumeric lower-kebab-case beginning with a letter.\n"))]
    pub word: String,
    /// # Operand Offest
    pub operand_parser_offset: u8,
    /// # Description
    /// Brief description of the opcode.
    #[serde(default)]
    #[validate(regex(path = "REGEX_RAIN_STRING", message = "Must be printable ASCII characters and whitespace.\n"))]
    pub description: String,
}

impl AuthoringMetaItem {
    pub fn abi_encode(&self) -> anyhow::Result<Vec<u8>> {
        Ok(AuthoringMetaStruct::abi_encode(&(
            str_to_bytes32(self.word.as_str())?,
            self.operand_parser_offset,
            self.description.clone()
        )))
    }

    pub fn abi_encode_validate(&self) -> anyhow::Result<Vec<u8>> {
        self.validate()?;
        self.abi_encode()
    }

    pub fn abi_decode(data: &Vec<u8>) -> anyhow::Result<AuthoringMetaItem> {
        let result = AuthoringMetaStruct::abi_decode(data, false)?;
        Ok(AuthoringMetaItem { 
            word: bytes32_to_str(&result.0)?.to_string(), 
            operand_parser_offset: result.1, 
            description: result.2.to_string() 
        })
    }

    pub fn abi_decode_validate(data: &Vec<u8>) -> anyhow::Result<AuthoringMetaItem> {
        let result = AuthoringMetaStruct::abi_decode(data, true)?;
        let am = AuthoringMetaItem { 
            word: bytes32_to_str(&result.0)?.to_string(), 
            operand_parser_offset: result.1, 
            description: result.2.to_string() 
        };
        am.validate()?;
        Ok(am)
    }
}


impl AuthoringMeta {
    
    /// abi encodes array of AuthoringMeta items
    pub fn abi_encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut v = vec![];
        for item in &self.0 {
            v.push((
                str_to_bytes32(item.word.as_str())?,
                item.operand_parser_offset,
                item.description.clone()
            ))
        };
        Ok(AuthoringMetaStructArray::abi_encode(&v))
    }

    /// abi encodes array of AuthoringMeta items after validating each
    pub fn abi_encode_validate(&self) -> anyhow::Result<Vec<u8>> {
        self.validate()?;
        self.abi_encode()
    }

    /// abi decodes some data into array of AuthoringMeta
    pub fn abi_decode(data: &Vec<u8>) -> anyhow::Result<AuthoringMeta> {
        let result = AuthoringMetaStructArray::abi_decode(data, false)?;
        let mut am = vec![];
        for item in result {
            am.push(AuthoringMetaItem { 
                word: bytes32_to_str(&item.0)?.to_string(), 
                operand_parser_offset: item.1, 
                description: item.2.to_string() 
            });
        };
        Ok(AuthoringMeta(am))
    }

    /// abi decodes some data into array of AuthoringMeta and validates each decoded item
    pub fn abi_decode_validate(data: &Vec<u8>) -> anyhow::Result<AuthoringMeta> {
        let result = AuthoringMetaStructArray::abi_decode(data, true)?;
        let mut ams = vec![];
        for item in result {
            ams.push(AuthoringMetaItem { 
                word: bytes32_to_str(&item.0)?.to_string(), 
                operand_parser_offset: item.1, 
                description: item.2.to_string() 
            });
        };
        let am = AuthoringMeta(ams);
        am.validate()?;
        Ok(am)
    }
}

impl Validate for AuthoringMeta {
    fn validate(&self) -> Result<(), ValidationErrors> {
        ValidationErrors::merge_all(
            Ok(()),
            "root",
            self.0.iter().map(|item| item.validate()).collect()
        )
    }
}

impl TryFrom<Vec<u8>> for AuthoringMeta {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match AuthoringMeta::abi_decode(&value.to_vec()) {
            Ok(am) => Ok(am),
            Err(_e) => serde_json::from_str::<AuthoringMeta>(
                std::str::from_utf8(&value).or(Err(anyhow::anyhow!(
                    "deserialization attempts failed with both abi decoding and json parsing"
                )))?
            ).or(Err(anyhow::anyhow!(
                "deserialization attempts failed with both abi decoding and json parsing"
            )))
        }
    }
}

impl TryFrom<MetaMap> for AuthoringMeta {
    type Error = anyhow::Error;
    fn try_from(value: MetaMap) -> Result<Self, Self::Error> {
        AuthoringMeta::try_from(value.unpack()?)
    }
}


#[cfg(test)]
mod tests {
    use crate::utils;
    use alloy_sol_types::{SolType, sol};
    use super::{AuthoringMeta, AuthoringMetaItem};

    #[test]
    fn test_encode_decode_validate() -> anyhow::Result<()> {
        let authoring_meta_content = r#"[
            {
                "word": "stack",
                "description": "Copies an existing value from the stack.",
                "operandParserOffset": 16
            },
            {
                "word": "constant",
                "description": "Copies a constant value onto the stack.",
                "operandParserOffset": 16
            }
        ]"#;

        // check the deserialization
        let authoring_meta: AuthoringMeta = serde_json::from_str(authoring_meta_content)?;
        let expected_authoring_meta = AuthoringMeta(vec![
            AuthoringMetaItem{
                word: "stack".to_string(), 
                operand_parser_offset: 16u8, 
                description: "Copies an existing value from the stack.".to_string()
            }, 
            AuthoringMetaItem{
                word: "constant".to_string(), 
                operand_parser_offset: 16u8, 
                description: "Copies a constant value onto the stack.".to_string()
            }
        ]);
        assert_eq!(authoring_meta, expected_authoring_meta );

        // abi encode the authoring meta with performing validation
        let authoring_meta_abi_encoded = authoring_meta.abi_encode_validate()?;
        let expected_abi_encoded_data = <sol!((bytes32, uint8, string)[])>::abi_encode(&vec![
            (
                utils::str_to_bytes32("stack")?,
                16u8,
                "Copies an existing value from the stack.".to_string()
            ),
            (
                utils::str_to_bytes32("constant")?,
                16u8,
                "Copies a constant value onto the stack.".to_string()
            )
        ]);
        // check the encoded bytes agaiinst the expected
        assert_eq!(authoring_meta_abi_encoded, expected_abi_encoded_data);

        let authoring_meta_abi_decoded = AuthoringMeta::abi_decode_validate(&authoring_meta_abi_encoded)?;
        assert_eq!(authoring_meta_abi_decoded, expected_authoring_meta);

        Ok(())
    }
}