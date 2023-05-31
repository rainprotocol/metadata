use schemars::JsonSchema;
use regex::Regex;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use validator::Validate;

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
    Regex::new(r"^[a-z][a-z0-9]*([A-Z][a-z0-9]*)*$").unwrap()
});

/// Strings in Rain are limited to printable ASCII chars and whitespace.
pub const REGEX_RAIN_STRING: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[\s!-~]*$").unwrap()
});

/// Rain symbols are a subset of kebab case.
#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RainSymbol{
    #[validate(regex(path = "REGEX_RAIN_SYMBOL", message = "Must be alphanumeric lower-kebab-case beginning with a letter.\n"))]
    pub value: String
}

pub type Name = RainSymbol;

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PascalSymbol{
    #[validate(regex(path = "REGEX_PASCAL_SYMBOL", message = "Must be alphanumeric PascalCase beginning with a letter.\n"))]
    pub value: String
}

pub type SolidityContractName = PascalSymbol;

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CamelSymbol{
    #[validate(regex(path = "REGEX_CAMEL_SYMBOL", message = "Must be alphanumeric camelCase beginning with a letter.\n"))]
    pub value: String
}

pub type SolidityFunctionName = CamelSymbol;
pub type SoliditySymbol = CamelSymbol;

#[derive(Validate, JsonSchema, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RainString {
    #[validate(regex(path = "REGEX_RAIN_STRING", message = "Must be printable ASCII characters and whitespace.\n"))]
    pub value: String
}

pub type Description = RainString;

#[cfg(test)]
mod test {
    use super::RainSymbol;
    use super::PascalSymbol;
    use super::CamelSymbol;
    use super::RainString;
    use validator::Validate;

    #[test]
    fn test_rain_symbol_validate() {
        // valids
        for i in ["a", "a-", "a-a", "a0"] {
            assert!(RainSymbol{ value: i.to_string()}.validate().is_ok());
        }

        // invalids
        for i in ["", "♥", "-", " ", "A", "A0", "a ", "0", "_", "0a", "0A"] {
            assert!(RainSymbol{ value: i.to_string()}.validate().is_err());
        }
    }

    #[test]
    fn test_pascal_symbol_validate() {
        // valids
        for i in ["A", "AA", "A0", "OrderBook"] {
            assert!(PascalSymbol{ value: i.to_string()}.validate().is_ok());
        }

        // invalids
        for i in ["", "a", "a-", "a-a", "a0", "♥", "-", " ", "a ", "0", "_", "0a", "0A"] {
            assert!(PascalSymbol{ value: i.to_string()}.validate().is_err());
        }
    }

    #[test]
    fn test_camel_symbol_validate() {
        // valids
        for i in ["a", "aa", "aA", "aAa", "a0", "aa0", "aA0", "aA0a", "aA0a0"] {
            assert!(CamelSymbol{ value: i.to_string()}.validate().is_ok(), "String '{}' considered invalid.", i);
        }

        // invalids
        for i in ["", "a-", "a-a", "♥", "-", " ", "a ", "0", "_", "0a", "0A"] {
            assert!(CamelSymbol{ value: i.to_string()}.validate().is_err(), "String '{}' considered valid.", i);
        }
    }

    #[test]
    fn test_rain_string_validate() {
        // valids
        for i in ["a", "aa", "aA", "aAa", "a0", "aa0", "aA0", "aA0a", "aA0a0", "", "a-", "a-a", "-", " ", "a ", "0", "_", "0a", "0A", "`", "```", "\n", ":"] {
            assert!(RainString{ value: i.to_string()}.validate().is_ok(), "String '{}' considered invalid.", i);
        }

        // invalids
        for i in ["♥", "∴"] {
            assert!(CamelSymbol{ value: i.to_string()}.validate().is_err(), "String '{}' considered valid.", i);
        }
    }
}