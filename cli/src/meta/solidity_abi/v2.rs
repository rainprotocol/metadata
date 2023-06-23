use schemars::JsonSchema;
use validator::Validate;
use serde::Deserialize;
use serde::Serialize;
use validator::ValidationErrors;
use serde::Deserializer;
use serde::de::Error;
use serde::Serializer;

/// # SolidityABI
/// JSON representation of a Solidity ABI interface.
/// https://docs.soliditylang.org/en/latest/abi-spec.html#json
#[derive(JsonSchema, Debug, Serialize, Deserialize)]
pub struct SolidityAbi(Vec<SolidityAbiItem>);

impl Validate for SolidityAbi {
    fn validate(&self) -> Result<(), ValidationErrors> {
        ValidationErrors::merge_all(
            Ok(()),
            "root",
            self.0.iter().map(|item| item.validate()).collect()
        )
    }
}

#[derive(Serialize, Validate, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiItemFn {
    name: String,
    inputs: Vec<SolidityAbiFnIO>,
    outputs: Vec<SolidityAbiFnIO>,
    state_mutability: SolidityAbiFnMutability,
}

#[derive(Serialize, Validate, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiItemConstructor {
    inputs: Vec<SolidityAbiFnIO>,
    state_mutability: SolidityAbiFnMutability,
}

#[derive(Serialize, Validate, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiItemReceive {
    state_mutability: SolidityAbiFnMutability,
}

#[derive(Serialize, Validate, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiItemFallback {
    state_mutability: SolidityAbiFnMutability,
}

#[derive(Serialize, Validate, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiItemEvent {
    name: String,
    inputs: Vec<SolidityAbiEventInput>,
    anonymous: bool,
}

#[derive(Serialize, Validate, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SolidityAbiItemError {
    name: String,
    inputs: Vec<SolidityAbiErrorInput>,
}

#[derive(JsonSchema, Debug)]
pub enum SolidityAbiItem {
    Function(SolidityAbiItemFn),
    Constructor(SolidityAbiItemConstructor),
    Receive(SolidityAbiItemReceive),
    Fallback(SolidityAbiItemFallback),
    Event(SolidityAbiItemEvent),
    Error(SolidityAbiItemError),
}

impl Serialize for SolidityAbiItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SolidityAbiItem::Function(item_fn) => item_fn.serialize(serializer),
            SolidityAbiItem::Constructor(item_constructor) => item_constructor.serialize(serializer),
            SolidityAbiItem::Receive(item_receive) => item_receive.serialize(serializer),
            SolidityAbiItem::Fallback(item_fallback) => item_fallback.serialize(serializer),
            SolidityAbiItem::Event(item_event) => item_event.serialize(serializer),
            SolidityAbiItem::Error(item_error) => item_error.serialize(serializer),
        }
    }
}

impl Validate for SolidityAbiItem {
    fn validate(&self) -> Result<(), ValidationErrors> {
        match self {
            SolidityAbiItem::Function(item_fn) => item_fn.validate(),
            SolidityAbiItem::Constructor(item_constructor) => item_constructor.validate(),
            SolidityAbiItem::Receive(item_receive) => item_receive.validate(),
            SolidityAbiItem::Fallback(item_fallback) => item_fallback.validate(),
            SolidityAbiItem::Event(item_event) => item_event.validate(),
            SolidityAbiItem::Error(item_error) => item_error.validate(),
        }
    }
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
pub struct SolidityAbiFnIO {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    components: Vec<SolidityAbiFnIO>,
}

#[derive(JsonSchema, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SolidityAbiFnMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
pub struct SolidityAbiErrorInput {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    components: Vec<SolidityAbiErrorInput>,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
pub struct SolidityAbiEventInput {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    components: Vec<SolidityAbiEventInputComponent>,
    indexed: bool,
}

#[derive(Validate, JsonSchema, Debug, Serialize, Deserialize)]
pub struct SolidityAbiEventInputComponent {
    name: String,
    #[serde(rename = "type")]
    typ: String,
    components: Vec<SolidityAbiEventInputComponent>,
}

impl<'de> Deserialize<'de> for SolidityAbiItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct Intermediate {
            #[serde(rename = "type")]
            typ: IntermediateType,
            name: Option<String>,
            inputs: Option<Vec<IntermediateIO>>,
            outputs: Option<Vec<IntermediateIO>>,
            state_mutability: Option<SolidityAbiFnMutability>,
            anonymous: Option<bool>
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "lowercase")]
        enum IntermediateType {
            Function,
            Constructor,
            Receive,
            Fallback,
            Event,
            Error,
        }

        #[derive(Debug, Deserialize)]
        struct IntermediateIO {
            name: String,
            #[serde(rename = "type")]
            typ: String,
            components: Option<Vec<IntermediateIO>>,
            indexed: Option<bool>,
        }

        let intermediate = Intermediate::deserialize(deserializer)?;

        let map_item_fn_io;
        map_item_fn_io = |intermediate_io: IntermediateIO| -> Result<SolidityAbiFnIO, D::Error> {
            if intermediate_io.indexed.is_some() {
                return Err(D::Error::custom("indexed found on fn io"));
            }

            let components: Option<Result<Vec<SolidityAbiFnIO>, D::Error>> = match intermediate_io.components.map(|cs| {
                let cs.iter().map(map_item_fn_io).collect();
            }) {
                Some(Err(err))
                None
            }
            Ok(SolidityAbiFnIO {
                name: intermediate_io.name,
                typ: intermediate_io.typ,
                components: components?,
            })
        };

        match intermediate.typ {
            IntermediateType::Function => {
                Ok(SolidityAbiItem::Function(SolidityAbiItemFn {
                    name: intermediate.name.ok_or(D::Error::custom("function missing name"))?,
                    inputs: intermediate.inputs.iter().map(map_item_fn_io).collect(),
                    outputs: intermediate.outputs.iter().map(map_item_fn_io).collect(),
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("function missing mutability"))?,
                }))
            },
            IntermediateType::Constructor => {
                Ok(SolidityAbiItem::Constructor(SolidityAbiItemConstructor {
                    inputs: vec![],
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("constructor missing mutability"))?,
                }))
            },
            IntermediateType::Receive => {
                Ok(SolidityAbiItem::Receive(SolidityAbiItemReceive {
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("receive missing mutability"))?,
                }))
            },
            IntermediateType::Fallback => {
                Ok(SolidityAbiItem::Fallback(SolidityAbiItemFallback {
                    state_mutability: intermediate.state_mutability.ok_or(D::Error::custom("fallback missing mutability"))?,
                }))
            },
            IntermediateType::Event => {
                Ok(SolidityAbiItem::Event(SolidityAbiItemEvent {
                    name: intermediate.name.ok_or(D::Error::custom("event missing name"))?,
                    inputs: vec![],
                    anonymous: intermediate.anonymous.ok_or(D::Error::custom("event missing anonymous"))?,
                }))
            },
            IntermediateType::Error => {
                Ok(SolidityAbiItem::Error(SolidityAbiItemError {
                    name: intermediate.name.ok_or(D::Error::custom("error missing name"))?,
                    inputs: vec![],
                }))
            }
        }
    }
}