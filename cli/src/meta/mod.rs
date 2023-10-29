pub mod solidity_abi;
pub mod op;
pub mod interpreter_caller;
pub mod rain;
pub mod normalize;
pub mod magic;

use magic::KnownMagic;
use serde::ser::{Serialize, Serializer, SerializeMap};
use serde::de::{Deserialize, Deserializer, Visitor};

use strum::EnumIter;
use strum::EnumString;

#[derive(Copy, Clone, EnumString, EnumIter, strum::Display, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum KnownMeta {
    SolidityAbiV2,
    InterpreterCallerMetaV1,
    OpV1,
    AuthoringMetaV1,
    DotrainV1,
    RainlangV1,
    ExpressionDeployerV2BytecodeV1,
}

impl TryFrom<KnownMagic> for KnownMeta {
    type Error = anyhow::Error;
    fn try_from(magic: KnownMagic) -> anyhow::Result<Self> {
        match magic {
            KnownMagic::SolidityAbiV2 => Ok(KnownMeta::SolidityAbiV2),
            KnownMagic::InterpreterCallerMetaV1 => Ok(KnownMeta::InterpreterCallerMetaV1),
            KnownMagic::OpMetaV1 => Ok(KnownMeta::OpV1),
            KnownMagic::AuthoringMetaV1 => Ok(KnownMeta::AuthoringMetaV1),
            KnownMagic::DotrainV1 => Ok(KnownMeta::DotrainV1),
            KnownMagic::RainlangV1 => Ok(KnownMeta::RainlangV1),
            KnownMagic::ExpressionDeployerV2BytecodeV1 => Ok(KnownMeta::ExpressionDeployerV2BytecodeV1),
            _ => Err(anyhow::anyhow!("Unsupported magic {}", magic)),
        }
    }
}

#[derive(serde::Serialize, Copy, Clone, EnumString, EnumIter, strum::Display, Debug, PartialEq, serde::Deserialize)]
#[strum(serialize_all = "kebab-case")]
pub enum ContentType {
    None,
    #[serde(rename = "application/json")]
    Json,
    #[serde(rename = "application/cbor")]
    Cbor,
    #[serde(rename = "application/octet-stream")]
    OctetStream
}

#[derive(serde::Serialize, Copy, Clone, EnumString, EnumIter, strum::Display, Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ContentEncoding {
    None,
    Identity,
    Deflate,
}

impl ContentEncoding {
    /// encode the data based on the variant
    pub fn encode(&self, data: Vec<u8>) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            ContentEncoding::None | ContentEncoding::Identity => data,
            ContentEncoding::Deflate => deflate::deflate_bytes_zlib(&data),
        })
    }

    /// decode the data based on the variant
    pub fn decode(&self, data: Vec<u8>) -> anyhow::Result<Vec<u8>, String> {
        Ok(match self {
            ContentEncoding::None | ContentEncoding::Identity => data,
            ContentEncoding::Deflate => inflate::inflate_bytes_zlib(&data)?,
        })
    }
}

#[derive(serde::Serialize, Copy, Clone, EnumString, EnumIter, strum::Display, Debug, PartialEq, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ContentLanguage {
    None,
    En,
}

#[derive(PartialEq, Debug, Clone)]
pub struct RainMeta {
    pub payload: serde_bytes::ByteBuf,
    pub magic: KnownMagic,
    pub content_type: ContentType,
    pub content_encoding: ContentEncoding,
    pub content_language: ContentLanguage,
}

impl RainMeta {
    fn len(&self) -> usize {
        let mut l = 2;
        if !matches!(self.content_type, ContentType::None) {
            l += 1;
        }
        if !matches!(self.content_encoding, ContentEncoding::None) {
            l += 1;
        }
        if !matches!(self.content_language, ContentLanguage::None) {
            l += 1;
        }
        l
    }

    /// method to cbor encode
    pub fn cbor_encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = vec![];
        match serde_cbor::to_writer(&mut bytes, &self) {
            Ok(()) => Ok(bytes),
            Err(error) => Err(error)?
        }
    }

    /// method to cbor decode bytes
    pub fn cbor_decode(data: &Vec<u8>) -> anyhow::Result<Vec<RainMeta>> {
        let mut track: Vec<usize> = vec![];
        let mut metas: Vec<RainMeta> = vec![];
        let mut is_rain_document_meta = false;
        let mut len = data.len();
        if data.starts_with(&KnownMagic::RainMetaDocumentV1.to_prefix_bytes()) {
            is_rain_document_meta = true;
            len -= 8;
        }
        let mut deserializer = match is_rain_document_meta {
            true => serde_cbor::Deserializer::from_slice(&data[8..]),
            false => serde_cbor::Deserializer::from_slice(data),
        };
        while match serde_cbor::Value::deserialize(&mut deserializer) {
            Ok(cbor_map) => { 
                track.push(deserializer.byte_offset());
                match serde_cbor::value::from_value(cbor_map) {
                    Ok(meta_content) => metas.push(meta_content),
                    Err(error) => Err(error)?
                };
                true
            },
            Err(error) => {
                if error.is_eof() {
                    if error.offset() == len as u64 {
                        false
                    } else {
                        Err(error)?
                    }
                } else {
                    Err(error)?
                }
            },
        } {};

        if metas.len() == 0 || track.len() == 0 || track.len() != metas.len() || len as usize != track[track.len() - 1] {
            return Err(anyhow::anyhow!("corrupt meta"));
        }
        Ok(metas)
    }

    /// builds a cbor sequence from given MetaConetnt
    pub fn build_seq(seq: &Vec<Self>, magic: KnownMagic) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = magic.to_prefix_bytes().to_vec();
        for item in seq {
            serde_cbor::to_writer(&mut bytes, &item)?;
        }
        Ok(bytes)
    }

    // pub fn 
}

impl Serialize for RainMeta {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        map.serialize_entry(&0, &self.payload)?;
        map.serialize_entry(&1, &(self.magic as u64))?;
        match self.content_type {
            ContentType::None => { },
            content_type => map.serialize_entry(&2, &content_type)?,
        }
        match self.content_encoding {
            ContentEncoding::None => { },
            content_encoding => map.serialize_entry(&3, &content_encoding)?,
        }
        match self.content_language {
            ContentLanguage::None => { },
            content_language => map.serialize_entry(&4, &content_language)?,
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for RainMeta {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EncodedMap;
        impl<'de> Visitor<'de> for EncodedMap {
            type Value = RainMeta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("rain meta cbor encoded bytes")
            }

            fn visit_map<T: serde::de::MapAccess<'de>>(self, mut map: T) -> Result<Self::Value, T::Error> {
                let mut payload = None;
                let mut magic = None;
                let mut content_type = None;
                let mut content_encoding = None;
                let mut content_language = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        0 => payload = Some(map.next_value()?),
                        1 => magic = Some(map.next_value()?),
                        2 => content_type = Some(map.next_value()?),
                        3 => content_encoding = Some(map.next_value()?),
                        4 => content_language = Some(map.next_value()?),
                        _ => {}
                    }
                }
                let payload = payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;
                let magic = KnownMagic::from_u64(magic.ok_or_else(|| serde::de::Error::missing_field("magic"))?).unwrap();
                let content_type = content_type.or(Some(ContentType::None)).unwrap();
                let content_encoding = content_encoding.or(Some(ContentEncoding::None)).unwrap();
                let content_language = content_language.or(Some(ContentLanguage::None)).unwrap();

                Ok(RainMeta { payload, magic, content_type, content_encoding, content_language })
            }
        }
        deserializer.deserialize_map(EncodedMap)
    }
}

// pub async fn search(hash: &str) -> Future {
//     MetaContent { 
//         payload: serde_bytes::ByteBuf::from(vec![33, 99]), 
//         magic: KnownMagic::SolidityAbiV2, 
//         content_type: ContentType::None, 
//         content_encoding: ContentEncoding::Identity, 
//         content_language: ContentLanguage::En 
//     }.
// }