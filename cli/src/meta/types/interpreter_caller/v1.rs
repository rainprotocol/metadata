use schemars::JsonSchema;
use super::super::common::v1::RainTitle;
use super::super::common::v1::RainSymbol;
use super::super::common::v1::Description;
use super::super::common::v1::RainString;
use super::super::common::v1::SolidityIdentifier;
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;
type AbiPath = RainString;

/// # InterpreterCallerMeta
/// InterpreterCaller metadata used by Rainlang.
/// Supports `IInterpreterCallerV2` Solidity contracts.
/// Required info about a contract that receives expression in at least one of
/// its methods.
#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct InterpreterCallerMeta {
    /// # Name
    #[validate]
    pub name: RainTitle,
    /// # Contract ABI name
    /// Name of the contract corresponding to `contractName` feild in the abi.
    #[validate]
    pub abi_name: SolidityIdentifier,
    /// # Caller Description
    /// Name of the caller corresponding to `contractName` feild in the abi.
    #[serde(default)]
    #[validate]
    pub desc: Description,
    /// # Alias
    /// Alias of the caller used by Rainlang.
    #[serde(default)]
    #[validate]
    pub alias: Option<RainSymbol>,
    /// # Methods
    ///  Methods of the contract that receive at least one expression
    /// (EvaluableConfig) from arguments.
    #[validate(length(min = 1))]
    #[validate]
    pub methods: Vec<Method>,
}

impl TryFrom<Vec<u8>> for InterpreterCallerMeta {
    type Error = anyhow::Error;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        match serde_json::from_slice::<Self>(&value).map_err(anyhow::Error::from) {
            Ok(t) => match t.validate().map_err(anyhow::Error::from) {
                Ok(()) => Ok(t),
                Err(e) => Err(e),
            },
            Err(e) => Err(e)
        }
    }
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Method {
    /// # Method name
    #[validate]
    pub name: RainTitle,
    #[validate]
    pub abi_name: SolidityIdentifier,
    #[serde(default)]
    #[validate]
    pub desc: Description,
    #[validate(length(min = 1))]
    #[validate]
    pub inputs: Vec<MethodInput>,
    #[validate]
    pub expressions: Vec<Expression>,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct MethodInput {
    #[validate]
    pub name: RainTitle,
    #[validate]
    pub abi_name: SolidityIdentifier,
    #[serde(default)]
    #[validate]
    pub desc: Description,
    #[validate]
    pub path: AbiPath,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct Expression {
    #[validate]
    pub name: RainTitle,
    #[validate]
    pub abi_name: SolidityIdentifier,
    #[serde(default)]
    #[validate]
    pub desc: Description,
    #[validate]
    pub path: AbiPath,
    #[serde(default)]
    pub signed_context: bool,
    #[serde(default)]
    pub caller_context: bool,
    #[serde(default)]
    #[validate(length(max = "u8::MAX"))]
    #[validate]
    pub context_columns: Vec<ContextColumn>,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextColumn {
    #[validate]
    pub name: RainTitle,
    #[serde(default)]
    #[validate]
    pub desc: Description,
    #[serde(default)]
    #[validate]
    pub alias: Option<RainSymbol>,
    #[serde(default)]
    #[validate]
    pub cells: Vec<ContextCell>,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContextCell {
    #[validate]
    pub name: RainTitle,
    #[serde(default)]
    #[validate]
    pub desc: Description,
    #[serde(default)]
    #[validate]
    pub alias: Option<RainSymbol>
}