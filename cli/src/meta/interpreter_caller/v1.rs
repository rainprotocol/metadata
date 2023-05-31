use schemars::JsonSchema;
use crate::meta::rain::v1::Name;
use crate::meta::rain::v1::Description;
use crate::meta::rain::v1::SoliditySymbol;
use crate::meta::rain::v1::RainString;
use crate::meta::rain::v1::SolidityContractName;
use crate::meta::rain::v1::SolidityFunctionName;

type AbiPath = RainString;

/// # InterpreterCallerMeta
/// InterpreterCaller metadata used by Rainlang.
/// Supports `IInterpreterCallerV2` Solidity contracts.
/// Required info about a contract that receives expression in at least one of
/// its methods.
#[derive(JsonSchema, Debug)]
pub struct InterpreterCallerMeta {
    /// # Name
    pub name: Name,
    /// # Contract ABI name
    /// Name of the contract corresponding to `contractName` feild in the abi.
    pub abi_name: SolidityContractName,
    /// # Caller Description
    /// Name of the caller corresponding to `contractName` feild in the abi.
    #[serde(default)]
    pub desc: Description,
    /// # Alias
    /// Alias of the caller used by Rainlang.
    #[serde(default)]
    pub alias: Option<Name>,
    /// # Methods
    ///  Methods of the contract that receive at least one expression
    /// (EvaluableConfig) from arguments.
    #[validate(length(min = 1))]
    pub methods: Vec<Method>,
}

#[derive(JsonSchema, Debug)]
pub struct Method {
    /// # Method name
    pub name: Name,
    pub abi_name: SolidityFunctionName,
    #[serde(default)]
    pub desc: Description,
    #[validate(length(min = 1))]
    pub inputs: Vec<MethodInput>,
    pub expressions: Vec<Expression>,
}

#[derive(JsonSchema, Debug)]
pub struct MethodInput {
    pub name: Name,
    pub abi_name: SoliditySymbol,
    #[serde(default)]
    pub desc: Description,
    pub path: AbiPath,
}

#[derive(JsonSchema, Debug)]
pub struct Expression {
    pub name: Name,
    pub abi_name: SoliditySymbol,
    #[serde(default)]
    pub desc: Description,
    pub path: AbiPath,
    #[serde(default)]
    pub signed_context: bool,
    #[serde(default)]
    pub caller_context: bool,
    #[serde(default)]
    #[validate(length(max = "u8::MAX"))]
    pub context_columns: Vec<ContextColumn>,
}

#[derive(JsonSchema, Debug)]
pub struct ContextColumn {
    pub name: Name,
    #[serde(default)]
    pub desc: Description,
    #[serde(default)]
    pub alias: Option<Name>,
    #[serde(default)]
    pub cells: Vec<ContextCell>,
}

#[derive(JsonSchema, Debug)]
pub struct ContextCell {
    pub name: Name,
    #[serde(default)]
    pub desc: Description,
    #[serde(default)]
    pub alias: Option<Name>
}