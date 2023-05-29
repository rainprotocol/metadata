use schemars::JsonSchema;
use regex::Regex;
use once_cell::sync::Lazy;

pub type operand = u16;

pub const REGEX_VALID_SYMBOL: Lazy<Regex> = Lazy::new(|| {
    Regex::new("^[a-z][0-9a-z-]*$").unwrap()
});

#[derive(JsonSchema, Debug)]
#[validate(regex = "REGEX_VALID_SYMBOL")]
pub struct Name(pub String);

#[derive(JsonSchema, Debug)]
#[validate(range(min = 0, max = mem::size_of<operand>() - 1))]
pub struct BitInteger(pub u8);

#[derive(JsonSchema, Debug)]
/// # Opcode metadata used by Rainlang.
/// Schema for opcodes metadata used by Rainlang.
pub struct OpMeta {
    /// # Name
    /// Primary word used to identify the opcode.
    pub name: Name,
    /// # Description
    /// Brief description of the opcode.
    pub desc: String,
    /// # Operand
    /// Data required to calculate and format the operand.
    #[serde(default)]
    pub operand: Vec<OperandArg>,
    /// # Inputs
    /// Data required to specify the inputs of the opcode. 0 for opcodes with no
    /// input, for opcodes with constant number of inputs, the length of
    /// "parameters" array defines the number of inputs and for opcodes with
    /// dynamic number of inputs, "bits" field must be specified which determines
    /// this opcode has dynamic inputs and number of inputs will be derived from
    /// the operand bits with "computation" field applied if specified.
    #[serde(default)]
    pub inputs: Vec<Input>,
    /// # Outputs
    /// Data required to specify the outputs of the opcode. An integer specifies
    /// the number of outputs for opcodes with constants number of outputs and
    /// for opcodes with dynamic outputs the "bits" field will determine the
    /// number of outputs with "computation" field applied if specified.
    #[serde(default)]
    pub outputs: Vec<Output>,
    /// # Aliases
    /// Other words used to reference the opcode.
    #[serde(default)]
    pub aliases: Vec<Name>,
}

#[derive(JsonSchema, Debug)]
pub struct Input {
    #[serde(default)]
    pub parameters: Vec<InputParameter>,
    #[serde(default)]
    pub bits: Option<(BitInteger, BitInteger)>,
    #[serde(default)]
    pub computation: Option<String>,
}

#[derive(JsonSchema, Debug)]
pub struct InputParameter {
    name: Name,
    #[serde(default)]
    desc: String,
    #[serde(default)]
    spread: bool,
}

#[derive(JsonSchema, Debug)]
pub enum Output {
    Exact(u16),
}

#[derive(JsonSchema, Debug)]
pub struct OperandArg {
    /// # Allocated Operand Bits
    /// Specifies the bits to allocate to this operand argument.
    pub bits: (u8, u8),
    /// # Operand Argument Name
    /// Name of the operand argument. Argument with the name of "inputs" is
    /// reserved so that it wont be be typed inside <> and its value needed to
    /// construct the operand will be the number of items inside the opcode's
    /// parens (computation will apply to this value if provided).
    #[validate(regex = "REGEX_VALID_SYMBOL")]
    pub name: String,
    /// # Operand Argument Description
    /// Description of the operand argument.
    pub desc: Option<String>,
    /// # Allocated Operand Bits Computation
    /// Specifies any arithmetical operation that needs to be applied to the
    /// value of this operand argument. It will apply to the value before it be
    /// validated by the provided range. The "arg" keyword is reserved for
    /// accessing the value of this operand argument, example: "(arg + 1) * 2".
    pub computation: Option<String>,
    /// # Operand Argument Range
    /// Determines the valid range of the operand argument after computation
    /// applied. For example an operand argument can be any value between range
    /// of 1 - 10: [[1, 10]] or an operand argument can only be certain exact
    /// values: [[2], [3], [9]], meaning it can only be 2 or 3 or 9.
    pub valid_range: Option<Vec<OperandArgRange>>,
}

#[derive(JsonSchema, Debug)]
pub enum OperandArgRange {
    Exact(u16),
    Range(u16, u16),
}