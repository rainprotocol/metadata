use std::{collections::HashMap, vec};
use ethers::types::U256;
use eyre::Context;
use serde::{Deserialize, Serialize};

pub struct MagicNumbers {}

impl MagicNumbers {
    pub fn rain_meta_document() -> U256 {
        U256::from_str_radix("0xff0a89c674ee7874", 16).unwrap()
    }

    pub fn solidity_abiv2() -> U256 {
        U256::from_str_radix("0xffe5ffb4a3ff2cde", 16).unwrap()
    }

    pub fn ops_meta_v1() -> U256 {
        U256::from_str_radix("0xffe5282f43e495b4", 16).unwrap()
    }

    pub fn contract_meta_v1() -> U256 {
        U256::from_str_radix("0xffc21bbf86cc199b", 16).unwrap()
    }
}

#[derive(Debug)]
enum Value {
    StringValue(String),
    U256Value(U256),
}

pub fn encode_cbor(
    payload: String,
    magic_number: U256,
    content_type: String,
    content_encoding: Option<String>,
    content_language: Option<String>,
) -> () {
    // let mut map: HashMap<i32, Value> = HashMap::new();
    // map.insert(0, Value::StringValue(payload));
    // map.insert(1, Value::U256Value(magic_number));
    // map.insert(2, Value::StringValue(content_type));

    // match content_encoding {
    //     Some(val) => {
    //         map.insert(3, Value::StringValue(val));
    //     }
    //     None => {}
    // }

    // match content_language {
    //     Some(val) => {
    //         map.insert(4, Value::StringValue(val));
    //     }
    //     None => {}
    // }

    let data = vec![(0, payload), (1, magic_number.to_string()), (2, content_type)];
    
    let mut encoder = cbor::Encoder::from_memory();
    encoder.encode(&data).unwrap();
    println!("map : {:?}", String::from_utf8_lossy(encoder.as_bytes()));

    
}
