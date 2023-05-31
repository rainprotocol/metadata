use schemars::JsonSchema;
use regex::Regex;
use once_cell::sync::Lazy;

/// Operands in the standard interpreter are `u16` values.
pub type Operand = u16;

/// Valid symbols in Rainlang are alpha prefixed alphanumeric kebab case.
pub const REGEX_RAIN_SYMBOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z][0-9a-z-]*$").unwrap()
});

/// Solidity contract names are PascalCase.
pub const REGEX_PASCAL_SYMBOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z][a-z0-9]*)+$").unwrap()
});

/// Solidity contract function names are camelCase.
/// Identical to PascalCase regex with some leading lowercase alphanumerics.
pub const REGEX_CAMEL_SYMBOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z][a-z0-9]*([A-Z][a-z0-9]*)+$").unwrap()
});

/// Strings in Rain are limited to printable ASCII chars and whitespace.
pub const REGEX_RAIN_STRING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\s!-~]*$").unwrap()
});

/// Rain symbols are a subset of kebab case.
#[derive(JsonSchema, Debug)]
pub struct RainSymbol(
    #[validate(regex = "REGEX_RAIN_SYMBOL")]
    pub String
);

pub type Name = RainSymbol;

#[derive(JsonSchema, Debug)]
pub struct PascalSymbol(
    #[validate(regex = "REGEX_PASCAL_SYMBOL")]
    pub String
);

pub type SolidityContractName = PascalSymbol;

#[derive(JsonSchema, Debug)]
pub struct CamelSymbol(
    #[validate(regex = "REGEX_CAMEL_SYMBOL")]
    pub String
);

pub type SolidityFunctionName = CamelSymbol;
pub type SoliditySymbol = CamelSymbol;

#[derive(JsonSchema, Debug, Default)]
pub struct RainString(
    #[validate(regex = "REGEX_RAIN_STRING")]
    pub String
);

pub type Description = RainString;