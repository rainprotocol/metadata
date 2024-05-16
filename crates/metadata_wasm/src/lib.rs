use wasm_bindgen::prelude::*;
use rain_metadata::meta::{
                             RainMetaDocumentV1Item,
                             KnownMagic
                         };
use serde_wasm_bindgen::to_value;
use rain_metadata::Error;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, SerializeMap, Serializer};

/// method to cbor decode from given bytes
// #[wasm_bindgen]
    pub fn cbor_decode(data: &[u8]) -> Result<Vec<RainMetaDocumentV1Item>, Error> {
        let mut track: Vec<usize> = vec![];
        let mut metas: Vec<RainMetaDocumentV1Item> = vec![];
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
                dbg!(&cbor_map);
                match serde_cbor::value::from_value(cbor_map) {
                    Ok(meta) => metas.push(meta),
                    Err(error) => Err(Error::SerdeCborError(error))?,
                };
                true
            }
            Err(error) => {
                if error.is_eof() {
                    if error.offset() == len as u64 {
                        false
                    } else {
                        Err(Error::SerdeCborError(error))?
                    }
                } else {
                    Err(Error::SerdeCborError(error))?
                }
            }
        } {}

        if metas.is_empty()
            || track.is_empty()
            || track.len() != metas.len()
            || len != track[track.len() - 1]
        {
            Err(Error::CorruptMeta)?
        }
        Ok(metas)
    }