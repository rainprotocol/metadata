use serde::Serialize;
use serde::Deserialize;
use validator::Validate;
use schemars::JsonSchema;
use validator::ValidationErrors;
use ethers::{abi, utils, abi::Token, types::U256};

use super::super::common::v1::RainSymbol;
use crate::meta::types::common::v1::RainString;
use crate::meta::types::common::v1::Description;

/// authoring meta struct
pub const AUTHORING_META_STRUCT: &str = "(bytes32, uint8, string)[]";

/// # Authoring Meta
/// array of native parser opcode metadata
#[derive(JsonSchema, Debug, Serialize, Deserialize)]
pub struct AuthoringMeta(Vec<AuthoringMetaItem>);

/// AuthoringMeta single item
#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthoringMetaItem {
    /// # Word
    /// Primary word used to identify the opcode.
    #[validate]
    pub word: RainSymbol,
    /// # Operand Offest
    pub operand_parser_offset: u8,
    /// # Description
    /// Brief description of the opcode.
    #[validate]
    #[serde(default)]
    pub description: Description,
}

impl AuthoringMeta {
    /// abi encodes array of AuthoringMeta items after validating each
    pub fn abi_encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut tokens: Vec<Token> = vec![];
        for item in &self.0 {
            item.validate()?;
            tokens.push(Token::Tuple(
                vec![
                    Token::FixedBytes(utils::format_bytes32_string(item.word.value.as_str())?.to_vec()), 
                    Token::Uint(U256::from(item.operand_parser_offset)), 
                    Token::String(item.description.value.clone())
                ]
            ));
        }
        Ok(abi::encode(&[Token::Array(tokens)]))
    }

    /// abi decodes some data into array of AuthoringMeta and validates each decoded item
    pub fn abi_decode(data: &Vec<u8>) -> anyhow::Result<AuthoringMeta> {
        let params = abi::HumanReadableParser::parse_type("(bytes32, uint8, string)[]")?;
        let tokens = abi::decode(&[params], data)?;
        match &tokens[0] {
            Token::Array(tuples) => {
                let mut ama: Vec<AuthoringMetaItem> = vec![];
                for (index, tuple) in tuples.iter().enumerate() {
                    match tuple {
                        Token::Tuple(t) => {
                            let mut word = String::new();
                            let mut description = String::new();
                            let mut operand_parser_offset: u8 = 0;
                            match &t[0] {
                                Token::FixedBytes(bytes) => {
                                    word = utils::parse_bytes32_string(bytes.as_slice().try_into()?)?.to_string();
                                },
                                other => Err(anyhow::anyhow!("unexpected token type at index {index}, expected FixedBytes, got {}", other.to_string()))?
                            }
                            match &t[1] {
                                Token::Uint(uint) => {
                                    if uint.gt(&U256::from(255)) {
                                        Err(anyhow::anyhow!("operand value out of range of uint8, {:?}", uint))?
                                    } else {
                                        operand_parser_offset = uint.byte(0);
                                    }
                                },
                                other => Err(anyhow::anyhow!("unexpected token type at index {index}, expected Uint, got {}", other.to_string()))?
                            }
                            match &t[2] {
                                Token::String(str) => {
                                    description = str.clone();
                                },
                                other => Err(anyhow::anyhow!("unexpected token type at index {index}, expected String, got {}", other.to_string()))?
                            }
                            let am = AuthoringMetaItem { 
                                word: RainSymbol { value: word },
                                operand_parser_offset, 
                                description: RainString { value: description } 
                            };
                            am.validate()?;
                            ama.push(am)
                        },
                        other => Err(anyhow::anyhow!("unexpected token type at index {index}, expected Tuple, got {}", other.to_string()))?
                    }
                }
                Ok(AuthoringMeta(ama))
            },
            _ => Err(anyhow::anyhow!("invalid type"))?
        }
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
            Err(_e) => serde_json::from_str::<AuthoringMeta>(std::str::from_utf8(&value)?).map_err(anyhow::Error::from)
        }
    }
}