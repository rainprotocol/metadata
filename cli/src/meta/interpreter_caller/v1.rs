use schemars::JsonSchema;
use crate::meta::rain::v1::Name;
use crate::meta::rain::v1::ContractName;
use crate::meta::rain::v1::Description;

/// # InterpreterCallerMeta
/// InterpreterCaller metadata used by Rainlang.
/// Supports `IInterpreterCallerV2` Solidity contracts.
#[derive(JsonSchema, Debug)]
pub struct InterpreterCallerMeta {
    /// # Name
    name: Name,
    /// # Contract ABI name
    abi_name: ContractName,
    desc: Description,
    alias: Name,
    methods: Vec<Method>,
}

#[derive(JsonSchema, Debug)]
pub struct Method {

}