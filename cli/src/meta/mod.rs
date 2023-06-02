pub mod op;
pub mod interpreter_caller;
pub mod rain;
pub mod normalize;
pub mod magic;

use strum::EnumIter;
use strum::EnumString;

#[derive(Clone, EnumString, EnumIter, strum::Display)]
#[strum(serialize_all = "kebab_case")]
pub enum KnownMeta {
    InterpreterCallerV1,
    OpV1,
}

pub enum ContentType {
    ApplicationJson
}

pub enum ContentEncoding {
    Identity,
    Deflate,
}

pub enum ContentLanguage {
    En,
}

pub struct RainMetaDocumentV1Item {
    pub payload: Vec<u8>,
    pub magic: KnownMagic,
    pub content_type: Option<ContentType>,
    pub content_encoding: Option<ContentEncoding>,
    pub content_language: Option<ContentLanguage>,
}