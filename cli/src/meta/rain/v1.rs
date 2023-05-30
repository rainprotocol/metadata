use schemars::JsonSchema;
use regex::Regex;
use once_cell::sync::Lazy;

/// Operands in the standard interpreter are `u16` values.
pub type Operand = u16;

/// Valid symbols in Rainlang are alpha prefixed alphanumeric kebab case.
pub const REGEX_VALID_SYMBOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-z][0-9a-z-]*$").unwrap()
});

/// Solidity contracts are PascalCase by name.
pub const REGEX_VALID_CONTRACT_NAME: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([A-Z][a-z0-9]*)+$").unwrap()
});

/// Strings in Rain are limited to printable ASCII chars and whitespace.
pub const REGEX_VALID_RAIN_STRING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\s!-~]*$").unwrap()
});

/// # Name
/// Names must be a valid Rainlang symbol.
#[derive(JsonSchema, Debug)]
pub struct Name(
    #[validate(regex = "REGEX_VALID_SYMBOL")]
    pub String
);

/// # Contract Name
/// Solidity contract names must be valid PascalCase.
#[derive(JsonSchema, Debug)]
pub struct ContractName(
    #[validate(regex = "REGEX_VALID_CONTRACT_NAME")]
    pub String
);

/// # Description
/// Descriptions must be a valid Rainlang string.
#[derive(JsonSchema, Debug, Default)]
pub struct Description(
    #[validate(regex = "REGEX_VALID_RAIN_STRING")]
    pub String
);