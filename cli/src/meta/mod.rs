pub mod magic;
pub mod types;
pub mod query;
pub mod normalize;

use std::sync::Arc;
use std::fmt::Debug;
use std::convert::TryFrom;
use std::collections::HashMap;

use strum::EnumIter;
use strum::EnumString;

use reqwest::Client;
use futures::future;
use magic::KnownMagic;
use ethers::utils::keccak256;
use graphql_client::GraphQLQuery;
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer, SerializeMap};

pub use super::subgraph::KnownSubgraphs;


/// # Known Meta
/// all known meta identifiers
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
            _ => Err(anyhow::anyhow!("Unsupported meta {}", magic)),
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
    pub fn encode(&self, data: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            ContentEncoding::None | ContentEncoding::Identity => data.clone(),
            ContentEncoding::Deflate => deflate::deflate_bytes_zlib(data),
        })
    }

    /// decode the data based on the variant
    pub fn decode(&self, data: &Vec<u8>) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            ContentEncoding::None | ContentEncoding::Identity => data.clone(),
            ContentEncoding::Deflate => match inflate::inflate_bytes_zlib(data) {
                Ok(v) => v,
                Err(error) => Err(anyhow::anyhow!(error))?
            },
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

/// # Rain Meta Map
/// represents a meta map that can be cbor encoded or unpacked based on the fields configuration to the meta types
#[derive(PartialEq, Debug, Clone)]
pub struct MetaMap {
    pub payload: serde_bytes::ByteBuf,
    pub magic: KnownMagic,
    pub content_type: ContentType,
    pub content_encoding: ContentEncoding,
    pub content_language: ContentLanguage,
}


// this implementation is mainly used by Rainlang and Dotrain metas as they are aliased type for String
impl TryFrom<MetaMap> for String {
    type Error = anyhow::Error;
    fn try_from(value: MetaMap) -> Result<Self, Self::Error> {
        String::from_utf8(value.unpack()?).map_err(anyhow::Error::from)
    }
}

// this implementation is mainly used by ExpressionDeployerV2Bytecode metas as it is aliased type for Vec<u8>
impl TryFrom<MetaMap> for Vec<u8> {
    type Error = anyhow::Error;
    fn try_from(value: MetaMap) -> Result<Self, Self::Error> {
        value.unpack()
    }
}

impl MetaMap {
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

    /// method to hash(keccak256) the cbor encoded bytes of this instance
    pub fn hash(&self, as_rain_meta_document: bool) -> anyhow::Result<[u8; 32]> {
        if as_rain_meta_document {
            Ok(keccak256(Self::cbor_encode_seq(&vec![self.clone()], KnownMagic::RainMetaDocumentV1)?))
        } else {
            Ok(keccak256(self.cbor_encode()?))
        }
    }

    /// method to cbor encode
    pub fn cbor_encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = vec![];
        match serde_cbor::to_writer(&mut bytes, &self) {
            Ok(()) => Ok(bytes),
            Err(error) => Err(error)?
        }
    }

    /// builds a cbor sequence from given MetaMaps
    pub fn cbor_encode_seq(seq: &Vec<MetaMap>, magic: KnownMagic) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = magic.to_prefix_bytes().to_vec();
        for item in seq {
            serde_cbor::to_writer(&mut bytes, &item)?;
        }
        Ok(bytes)
    }

    /// method to cbor decode from given bytes
    pub fn cbor_decode(data: &Vec<u8>) -> anyhow::Result<Vec<MetaMap>> {
        let mut track: Vec<usize> = vec![];
        let mut metas: Vec<MetaMap> = vec![];
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
                    Ok(meta) => metas.push(meta),
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

        if metas.len() == 0 
           || track.len() == 0
           || track.len() != metas.len()
           || len as usize != track[track.len() - 1] 
        { return Err(anyhow::anyhow!("corrupt meta")); }
        Ok(metas)
    }

    // unpack the payload based on the configuration
    pub fn unpack(&self) -> anyhow::Result<Vec<u8>> {
        ContentEncoding::decode(&self.content_encoding, &self.payload.to_vec())
    }

    // unpacks the payload to given meta type based on configuration
    pub fn unpack_into<T: TryFrom<Self, Error = anyhow::Error>>(&self) -> anyhow::Result<T> {
        // let data = self.unpack()?;
        match self.magic {
            KnownMagic::OpMetaV1 |
            KnownMagic::DotrainV1 |
            KnownMagic::RainlangV1 |
            KnownMagic::SolidityAbiV2 |
            KnownMagic::AuthoringMetaV1 |
            KnownMagic::InterpreterCallerMetaV1 |
            KnownMagic::ExpressionDeployerV2BytecodeV1 => T::try_from(self.clone()).map_err(anyhow::Error::from),
            _ => Err(anyhow::anyhow!("unsupproted magic number"))
        }
    }
}

impl Serialize for MetaMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
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

impl<'de> Deserialize<'de> for MetaMap {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EncodedMap;
        impl<'de> Visitor<'de> for EncodedMap {
            type Value = MetaMap;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("rain meta cbor encoded bytes")
            }

            fn visit_map<T: serde::de::MapAccess<'de>>(self, mut map: T) -> Result<Self::Value, T::Error> {
                let mut payload = None;
                let mut magic: Option<u64> = None;
                let mut content_type = None;
                let mut content_encoding = None;
                let mut content_language = None;
                while match map.next_key() {
                    Ok(Some(key)) => {
                        match key {
                            0 => payload = Some(map.next_value()?),
                            1 => magic = Some(map.next_value()?),
                            2 => content_type = Some(map.next_value()?),
                            3 => content_encoding = Some(map.next_value()?),
                            4 => content_language = Some(map.next_value()?),
                            other => Err(serde::de::Error::custom(&format!("found unexpected key in the map: {other}")))?
                        };
                        true
                    },
                    Ok(None) => false,
                    Err(error) => Err(error)?
                } {};
                let payload = payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;
                let magic = match magic.ok_or_else(|| serde::de::Error::missing_field("magic number"))?.try_into() {
                    Ok(m) => m,
                    _ => Err(serde::de::Error::custom("unknown magic number"))?
                };
                let content_type = content_type.or(Some(ContentType::None)).unwrap();
                let content_encoding = content_encoding.or(Some(ContentEncoding::None)).unwrap();
                let content_language = content_language.or(Some(ContentLanguage::None)).unwrap();

                Ok(MetaMap { payload, magic, content_type, content_encoding, content_language })
            }
        }
        deserializer.deserialize_map(EncodedMap)
    }
}


/// # Search Meta
/// searches for a meta matching the given hash in given subgraphs urls
pub async fn search(hash: &str, subgraphs: &Vec<String>, timeout: u32) -> anyhow::Result<query::MetaResponse> {
    if !types::common::v1::HASH_PATTERN.is_match(hash) {
        return Err(anyhow::anyhow!("invalid hash"));
    }
    let request_body = query::MetaQuery::build_query(
        query::meta_query::Variables {
            hash: Some(hash.to_ascii_lowercase())
        }
    );
    let mut promises = vec![];
    let client = Arc::new(Client::builder().timeout(
        std::time::Duration::from_secs(timeout as u64)).build()?
    );
    for sg in subgraphs {
        promises.push(
            Box::pin(query::process_meta_query(client.clone(), &request_body, sg))
        );
    }
    let response_value = future::select_ok(promises.drain(..)).await?.0;

    if response_value.starts_with("0x") {
        Ok(query::MetaResponse{ bytes: hex::decode(&response_value[2..])? })
    } else {
        Ok(query::MetaResponse{ bytes: hex::decode(&response_value)? })
    }
}

/// # Search Deployer Meta
/// searches for a deployer meta matching the given hash in given subgraphs urls
pub async fn search_deployer(hash: &str, subgraphs: &Vec<String>, timeout: u32) -> anyhow::Result<query::DeployerMetaResponse> {
    if !types::common::v1::HASH_PATTERN.is_match(hash) {
        return Err(anyhow::anyhow!("invalid hash"));
    }
    let request_body = query::DeployerQuery::build_query(
        query::deployer_query::Variables {
            hash: Some(hash.to_ascii_lowercase())
        }
    );
    let mut promises = vec![];
    let client = Arc::new(Client::builder().timeout(
        std::time::Duration::from_secs(timeout as u64)).build()?
    );
    for sg in subgraphs {
        promises.push(
            Box::pin(query::process_deployer_query(client.clone(), &request_body, sg))
        );
    }
    let response_value = future::select_ok(promises.drain(..)).await?.0;

    if response_value.1.starts_with("0x") {
        Ok(query::DeployerMetaResponse{ 
            hash: response_value.0,
            bytes: hex::decode(&response_value.1[2..])? 
        })
    } else {
        Ok(query::DeployerMetaResponse{ 
            hash: response_value.0,
            bytes: hex::decode(&response_value.1)? 
        })
    }
}


/// # Meta Store(CAS)
/// 
/// Reads, stores and simply manages k/v pairs of meta hash and meta bytes and provides the functionalities 
/// to easliy utilize them. Hashes must be 32 bytes (in hex string format) and will be stored as lower case.
/// Meta items are stored as cbor encoded raw bytes.
/// 
/// Given a k/v pair of meta hash and meta bytes when using `update_with()` or `create()`,
/// it regenrates the hash from the corresponding bytes to check the validity of the given k/v pair and ignores
/// those that fail the check
/// 
/// ## Examples
/// 
/// ```rust
/// use rain_meta::meta::Store;
/// use std::collections::HashMap;
/// 
/// // to instantiate with including default subgraphs
/// let mut store = Store::new();
/// 
/// // to instatiate with default rain subgraphs included
/// let mut store = Store::default();
/// 
/// // or to instantiate with initial values
/// let mut store = Store::create(
///     &vec!["sg-url-1".to_string()], 
///     &HashMap::new(), 
///     &HashMap::new(), 
///     &HashMap::new(), 
///     true
/// );
/// 
/// // add a new subgraph endpoint url to the subgraph list
/// store.add_subgraphs(&vec!["sg-url-2".to_string()]);
/// 
/// // update the store with another Store (merges the stores)
/// store.merge(&Store::default());
/// 
/// // updates the meta store with a new meta by searching through subgraphs
/// store.update(&"some-hash".to_string());
/// 
/// // updates the meta store with a new meta hash and bytes
/// store.update_with(&"some-hash".to_string(), &vec![0u8, 1u8]);
/// 
/// // to get a record from store
/// let meta = store.get_meta(&"some-hash".to_string());
/// 
/// // to get a authoring meta record from store
/// let am = store.get_authoring_meta(&"some-hash".to_string());
/// 
/// // updates the dotrain cache for a dotrain text and uri
/// let (new_hash, old_hash) = store.set_dotrain(
///     &"some .rain text".to_string(), 
///     &".rain document uri".to_string(), 
///     false
/// ).unwrap();
/// 
/// // to get dotrain meta bytes given a uri
/// let dotrain_meta_bytes = store.get_dotrain_meta(&".rain document uri".to_string());
/// ```
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Store {
    subgraphs: Vec<String>,
    cache: HashMap<String, Vec<u8>>,
    dotrain_cache: HashMap<String, String>,
    authoring_cache: HashMap<String, Vec<u8>>,
}

impl Default for Store {
    fn default() -> Self {
        Store { 
            cache: HashMap::new(), 
            dotrain_cache: HashMap::new(), 
            authoring_cache: HashMap::new(), 
            subgraphs: KnownSubgraphs::NP.map(|url| url.to_string()).to_vec() 
        }
    }
}

impl Store {

    /// lazily creates a new instance
    /// it is recommended to use create() instead with initial values
    pub fn new() -> Store { 
        Store { 
            subgraphs: vec![],
            cache: HashMap::new(), 
            dotrain_cache: HashMap::new(), 
            authoring_cache: HashMap::new()
        }
    }

    /// creates new instance of Store with given initial values
    /// it checks the validity of each item of the provided values and only stores those that are valid
    pub fn create(
        subgraphs: &Vec<String>, 
        cache: &HashMap<String, Vec<u8>>, 
        authoring_cache: &HashMap<String, Vec<u8>>, 
        dotrain_cache: &HashMap<String, String>,
        include_rain_subgraphs: bool
    ) -> Store {
        let mut store;
        if include_rain_subgraphs {
            store = Store::default();
        } else {
            store = Store::new();
        }
        store.add_subgraphs(&subgraphs);
        for (hash, bytes) in cache { store.update_with(hash, bytes); }
        for (hash, bytes) in authoring_cache { 
            if types::common::v1::HASH_PATTERN.is_match(hash) {
                let _h = hash.to_ascii_lowercase();
                if !store.authoring_cache.contains_key(&_h) {
                    if let Ok(hash_bytes) = hex::decode(&_h) {
                        if keccak256(bytes) == hash_bytes.as_slice() {
                            store.authoring_cache.insert(_h, bytes.clone());
                        }
                    }
                }
            }
        }
        for (uri, hash) in dotrain_cache { 
            if types::common::v1::HASH_PATTERN.is_match(hash) {
                let _h = hash.to_ascii_lowercase();
                if !store.dotrain_cache.contains_key(uri) {
                    if store.cache.contains_key(&_h) {
                        store.dotrain_cache.insert(uri.clone(), _h);
                    }
                }
            }
        }
        store
    }

    /// all subgraph endpoints in this instance
    pub fn subgraphs(&self) -> &Vec<String> { &self.subgraphs }

    /// add new subgraph endpoints
    pub fn add_subgraphs(&mut self, subgraphs: &Vec<String>) {
        for sg in subgraphs {
            if !self.subgraphs.contains(sg) { self.subgraphs.push(sg.clone()); }
        }
    }

    /// getter method for the whole meta cache
    pub fn cache(&self) -> &HashMap<String, Vec<u8>> { &self.cache }

    /// get the corresponding meta bytes of the given hash if it exists
    pub fn get_meta(&self, hash: &String) -> Option<&Vec<u8>> {
        self.cache.get(&hash.to_ascii_lowercase())
    }

    /// getter method for the whole authoring meta cache
    pub fn authoring_cache(&self) -> &HashMap<String, Vec<u8>> { &self.authoring_cache }

    /// get the corresponding authoring meta bytes of the given hash if it exists
    pub fn get_authoring_meta(&self, hash: &String) -> Option<&Vec<u8>> {
        self.authoring_cache.get(&hash.to_ascii_lowercase())
    }

    /// if the authoring meta already is cached it returns it immediately else 
    /// searches for authoring meta in the subgraphs given the deployer hash
    pub async fn search_authoring_meta(&mut self, hash: &String) -> Option<&Vec<u8>> {
        if self.authoring_cache.contains_key(&hash.to_ascii_lowercase()) {
            self.get_authoring_meta(hash)
        } else {
            match search_deployer(hash, &self.subgraphs, 6u32).await {
                Ok(res) => self.update_with(&res.hash, &res.bytes),
                Err(_e) => None
            }
        }
    }

    /// getter method for the whole dotrain cache
    pub fn dotrain_cache(&self) -> &HashMap<String, String> { &self.dotrain_cache }

    /// get the corresponding dotrain hash of the given dotrain uri if it exists
    pub fn get_dotrain_hash(&self, uri: &String) -> Option<&String> {
        self.dotrain_cache.get(uri)
    }

    /// get the corresponding uri of the given dotrain hash if it exists
    pub fn get_dotrain_uri(&self, hash: &String) -> Option<&String> {
        for (uri, h) in &self.dotrain_cache {
            if h.eq_ignore_ascii_case(hash) { return Some(uri); }
        }
        return None;
    }

    /// get the corresponding meta bytes of the given dotrain uri if it exists
    pub fn get_dotrain_meta(&self, uri: &String) -> Option<&Vec<u8>> {
        self.get_meta(self.dotrain_cache.get(uri)?)
    }

    /// deletes a dotrain record given a uri
    pub fn delete_dotrain(&mut self, uri: &String, keep_meta: bool) {
        if let Some(kv) = self.dotrain_cache.remove_entry(uri) {
            if !keep_meta { self.cache.remove(&kv.1); }
        };
    }

    /// lazilly merges another Store to the current one, avoids duplicates
    pub fn merge(&mut self, other: &Store) {
        self.add_subgraphs(&other.subgraphs);
        for (hash, bytes) in &other.cache {
            if !self.cache.contains_key(hash) { 
                self.cache.insert(hash.to_ascii_lowercase(), bytes.clone()); 
            }
        }
        for (hash, bytes) in &other.authoring_cache {
            if !self.authoring_cache.contains_key(hash) { 
                self.authoring_cache.insert(hash.to_ascii_lowercase(), bytes.clone()); 
            }
        }
        for (uri, hash) in &other.dotrain_cache {
            if !self.dotrain_cache.contains_key(uri) { 
                self.dotrain_cache.insert(uri.clone(), hash.to_ascii_lowercase()); 
            }
        }
    }

    /// updates the meta cache by searching through all subgraphs for the given hash
    /// returns the reference to the authoring bytes if the updated meta bytes contained any
    pub async fn update(&mut self, hash: &String) -> Option<&Vec<u8>> {
        let mut am_bytes: Option<&Vec<u8>> = None;
        if types::common::v1::HASH_PATTERN.is_match(hash) {
            let _h = hash.to_ascii_lowercase();
            if !self.cache.contains_key(&_h) {
                if let Ok(meta) = search(hash, &self.subgraphs, 6u32).await {
                    self.cache.insert(_h, meta.bytes.clone());
                    am_bytes = self.store_content(&meta.bytes);
                };
            }
        }
        am_bytes
    }

    /// updates the meta cache by the given hash and meta bytes, checks the hash to bytes validity
    /// returns the reference to the authoring bytes if the updated meta bytes contained any
    pub fn update_with(&mut self, hash: &String, bytes: &Vec<u8>) -> Option<&Vec<u8>> {
        let mut am_bytes: Option<&Vec<u8>> = None;
        if types::common::v1::HASH_PATTERN.is_match(hash) {
            let _h = hash.to_ascii_lowercase();
            if !self.cache.contains_key(&_h) {
                if let Ok(hash_bytes) = hex::decode(&_h) {
                    if keccak256(bytes) == hash_bytes.as_slice() {
                        self.cache.insert(_h, bytes.clone());
                        am_bytes = self.store_content(bytes);
                    }
                }
            }
        }
        am_bytes
    }

    /// stores (or updates in case the URI already exists) the given dotrain text as meta into the store cache 
    /// and maps it to the given uri (path), it should be noted that reading the content of the dotrain is not in 
    /// the scope of Store and handling and passing on a correct URI (path) for the given text must be handled 
    /// externally by the implementer
    pub fn set_dotrain(&mut self, text: &String, uri: &String, keep_old: bool) -> anyhow::Result<(String, String)> {
        let bytes = MetaMap {
            payload: serde_bytes::ByteBuf::from(text.as_bytes()),
            magic: KnownMagic::DotrainV1,
            content_type: ContentType::OctetStream,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::None
        }.cbor_encode()?;
        let new_hash = "0x".to_owned() + &hex::encode(keccak256(&bytes));
        if let Some(k) = self.dotrain_cache.get(uri) {
            let old_hash = k.clone();
            if new_hash.eq_ignore_ascii_case(&old_hash) {
                self.cache.insert(new_hash.clone(), bytes);
                return Ok((new_hash, "".to_string()));
            } 
            else {
                self.cache.insert(new_hash.clone(), bytes);
                self.dotrain_cache.insert(uri.clone(), new_hash.clone());
                if !keep_old { self.cache.remove(&old_hash.clone()); }
                return Ok((new_hash, old_hash))
            }
        } else {
            self.dotrain_cache.insert(uri.clone(), new_hash.clone());
            self.cache.insert(new_hash.clone(), bytes);
            return Ok((new_hash, "".to_string()));
        };
    }

    /// decodes each meta and stores the inner meta items into the cache
    /// if any of the inner items is an authoring meta, stores it in authoring meta cache as well
    /// returns the reference to the authoring bytes if the meta bytes contained any
    fn store_content(&mut self, bytes: &Vec<u8>) -> Option<&Vec<u8>> {
        let mut h = String::new();
        if let Ok(meta_maps) = MetaMap::cbor_decode(bytes) {
            if bytes.starts_with(&KnownMagic::RainMetaDocumentV1.to_prefix_bytes()) {
                for meta_map in &meta_maps {
                    if let Ok(encoded_bytes) = meta_map.cbor_encode() {
                        let hash = "0x".to_owned() + &hex::encode(keccak256(&encoded_bytes));
                        h = hash.clone();
                        self.update_with(&hash, &encoded_bytes);
                        if meta_map.magic == KnownMagic::AuthoringMetaV1 {
                            self.authoring_cache.insert(hash.clone(), encoded_bytes);
                        }
                    }
                }
            } else {
                if meta_maps.len() == 1 && meta_maps[0].magic == KnownMagic::AuthoringMetaV1 {
                    let hash = "0x".to_owned() + &hex::encode(keccak256(bytes));
                    h = hash.clone();
                    self.authoring_cache.insert(hash.clone(), bytes.clone());
                }
            }
        }
        self.authoring_cache.get(&h)
    }
}


#[cfg(test)]
mod tests {
    use super::{
        MetaMap, 
        ContentType, 
        ContentEncoding, 
        ContentLanguage, 
        magic::KnownMagic, 
        types::{
            dotrain::v1::DotrainMeta,
            common::v1::{RainSymbol, Description}, 
            authoring::v1::{AuthoringMetaItem, AuthoringMeta}
        }
    };
    use ethers::{abi::{self, Token}, utils, types::U256};

    /// Roundtrip test for an authoring meta
    /// original content -> pack -> MetaMap -> cbor encode -> cbor decode -> MetaMap -> unpack -> original content,
    #[test]
    fn authoring_meta_roundtrip() -> anyhow::Result<()> {
        let authoring_meta_content = r#"[
            {
                "word": "stack",
                "description": "Copies an existing value from the stack.",
                "operandParserOffset": 16
            },
            {
                "word": "constant",
                "description": "Copies a constant value onto the stack.",
                "operandParserOffset": 16
            }
        ]"#;

        // check the deserialization
        let authoring_meta: AuthoringMeta = serde_json::from_str(authoring_meta_content)?;

        assert_eq!(
            authoring_meta, 
            AuthoringMeta(vec![
                AuthoringMetaItem{
                    word: RainSymbol{ value: "stack".to_string() }, 
                    operand_parser_offset: 16u8, 
                    description: Description{ value: "Copies an existing value from the stack.".to_string() }
                }, 
                AuthoringMetaItem{
                    word: RainSymbol{ value: "constant".to_string() }, 
                    operand_parser_offset: 16u8, 
                    description: Description{ value: "Copies a constant value onto the stack.".to_string() }
                }
            ])
        );

        // abi encode the authoring meta with performing validation
        let authoring_meta_abi_encoded = authoring_meta.abi_encode_validate()?;
        let expected_abi_encoded_data = abi::encode(&[Token::Array(vec![
            Token::Tuple(vec![
                Token::FixedBytes(utils::format_bytes32_string("stack")?.to_vec()), 
                Token::Uint(U256::from(16u8)), 
                Token::String("Copies an existing value from the stack.".to_string())
            ]),
            Token::Tuple(vec![
                Token::FixedBytes(utils::format_bytes32_string("constant")?.to_vec()), 
                Token::Uint(U256::from(16u8)), 
                Token::String("Copies a constant value onto the stack.".to_string())
            ])
        ])]);

        // check the encoded bytes agaiinst the expected
        assert_eq!(authoring_meta_abi_encoded, expected_abi_encoded_data);

        let meta_map = MetaMap{
            payload: serde_bytes::ByteBuf::from(authoring_meta_abi_encoded.clone()),
            magic: KnownMagic::AuthoringMetaV1,
            content_type: ContentType::Cbor,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::None
        };
        let cbor_encoded = meta_map.cbor_encode()?;

        // cbor map with 3 keys
        assert_eq!(cbor_encoded[0], 0xa3);
        // key 0
        assert_eq!(cbor_encoded[1], 0x00);
        // major type 2 (bytes) length 512
        assert_eq!(cbor_encoded[2], 0b010_11001);
        assert_eq!(cbor_encoded[3], 0b000_00010);
        assert_eq!(cbor_encoded[4], 0b000_00000);
        // payload
        assert_eq!(cbor_encoded[5..517], authoring_meta_abi_encoded);
        // key 1
        assert_eq!(cbor_encoded[517], 0x01);
        // major type 0 (unsigned integer) value 27
        assert_eq!(cbor_encoded[518], 0b000_11011);
        // magic number
        assert_eq!(&cbor_encoded[519..527], KnownMagic::AuthoringMetaV1.to_prefix_bytes());
        // key 2
        assert_eq!(cbor_encoded[527], 0x02);
        // text string application/cbor length 16
        assert_eq!(cbor_encoded[528], 0b011_10000);
        // the string application/cbor, must be the end of data
        assert_eq!(&cbor_encoded[529..], "application/cbor".as_bytes());

        // decode the data back to MetaMap
        let cbor_decoded = MetaMap::cbor_decode(&cbor_encoded)?;
        // the length of decoded maps must be 1 as we only had 1 encoded item
        assert_eq!(cbor_decoded.len(), 1usize);
        // decoded item must be equal to the original meta_map
        assert_eq!(cbor_decoded[0], meta_map);

        // unpack the payload into AuthoringMeta
        let unpacked_payload: AuthoringMeta = cbor_decoded[0].unpack_into()?;
        // must be equal to original meta
        assert_eq!(unpacked_payload, authoring_meta);
        
        Ok(())
    }

    /// Roundtrip test for a dotrain meta
    /// original content -> pack -> MetaMap -> cbor encode -> cbor decode -> MetaMap -> unpack -> original content,
    #[test]
    fn dotrain_meta_roundtrip() -> anyhow::Result<()> {
        let dotrain_content = "#main _ _: int-add(1 2) int-add(2 3)";
        let dotrain_content_bytes = dotrain_content.as_bytes().to_vec();

        let content_encoding = ContentEncoding::Deflate;
        let deflated_payload = content_encoding.encode(&dotrain_content_bytes)?;

        let meta_map = MetaMap{
            payload: serde_bytes::ByteBuf::from(deflated_payload.clone()),
            magic: KnownMagic::DotrainV1,
            content_type: ContentType::OctetStream,
            content_encoding,
            content_language: ContentLanguage::En
        };
        let cbor_encoded = meta_map.cbor_encode()?;

        // cbor map with 5 keys
        assert_eq!(cbor_encoded[0], 0xa5);
        // key 0
        assert_eq!(cbor_encoded[1], 0x00);
        // major type 2 (bytes) length 36
        assert_eq!(cbor_encoded[2], 0b010_11000);
        assert_eq!(cbor_encoded[3], 0b001_00100);
        // assert_eq!(cbor_encoded[4], 0b000_00000);
        // payload
        assert_eq!(cbor_encoded[4..40], deflated_payload);
        // key 1
        assert_eq!(cbor_encoded[40], 0x01);
        // major type 0 (unsigned integer) value 27
        assert_eq!(cbor_encoded[41], 0b000_11011);
        // magic number
        assert_eq!(&cbor_encoded[42..50], KnownMagic::DotrainV1.to_prefix_bytes());
        // key 2
        assert_eq!(cbor_encoded[50], 0x02);
        // text string application/octet-stream length 24
        assert_eq!(cbor_encoded[51], 0b011_11000);
        assert_eq!(cbor_encoded[52], 0b000_11000);
        // the string application/octet-stream
        assert_eq!(&cbor_encoded[53..77], "application/octet-stream".as_bytes());
        // key 3
        assert_eq!(cbor_encoded[77], 0x03);
        // text string deflate length 7
        assert_eq!(cbor_encoded[78], 0b011_00111);
        // the string deflate
        assert_eq!(&cbor_encoded[79..86], "deflate".as_bytes());
        // key 4
        assert_eq!(cbor_encoded[86], 0x04);
        // text string en length 2
        assert_eq!(cbor_encoded[87], 0b011_00010);
        // the string identity, must be the end of data
        assert_eq!(&cbor_encoded[88..], "en".as_bytes());

        // decode the data back to MetaMap
        let cbor_decoded = MetaMap::cbor_decode(&cbor_encoded)?;
        // the length of decoded maps must be 1 as we only had 1 encoded item
        assert_eq!(cbor_decoded.len(), 1usize);
        // decoded item must be equal to the original meta_map
        assert_eq!(cbor_decoded[0], meta_map);

        // unpack the payload into DotrainMeta, should handle inflation of the payload internally
        let unpacked_payload: DotrainMeta = cbor_decoded[0].unpack_into()?;
        // must be equal to the original dotrain content
        assert_eq!(unpacked_payload, dotrain_content);
        
        Ok(())
    }

    /// Roundtrip test for a meta sequence
    /// original content -> pack -> MetaMap -> cbor encode -> cbor decode -> MetaMap -> unpack -> original content,
    #[test]
    fn meta_seq_roundtrip() -> anyhow::Result<()> {
        let authoring_meta_content = r#"[
            {
                "word": "stack",
                "description": "Copies an existing value from the stack.",
                "operandParserOffset": 16
            },
            {
                "word": "constant",
                "description": "Copies a constant value onto the stack.",
                "operandParserOffset": 16
            }
        ]"#;
        let authoring_meta: AuthoringMeta = serde_json::from_str(authoring_meta_content)?;
        let authoring_meta_abi_encoded = authoring_meta.abi_encode_validate()?;
        let meta_map_1 = MetaMap{
            payload: serde_bytes::ByteBuf::from(authoring_meta_abi_encoded.clone()),
            magic: KnownMagic::AuthoringMetaV1,
            content_type: ContentType::Cbor,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::None
        };

        let dotrain_content = "#main _ _: int-add(1 2) int-add(2 3)";
        let dotrain_content_bytes = dotrain_content.as_bytes().to_vec();
        let content_encoding = ContentEncoding::Deflate;
        let deflated_payload = content_encoding.encode(&dotrain_content_bytes)?;
        let meta_map_2 = MetaMap{
            payload: serde_bytes::ByteBuf::from(deflated_payload.clone()),
            magic: KnownMagic::DotrainV1,
            content_type: ContentType::OctetStream,
            content_encoding,
            content_language: ContentLanguage::En
        };

        // cbor encode as RainMetaDocument sequence
        let cbor_encoded = MetaMap::cbor_encode_seq(
            &vec![meta_map_1.clone(), meta_map_2.clone()], 
            KnownMagic::RainMetaDocumentV1
        )?;

        // 8 byte magic number prefix
        assert_eq!(&cbor_encoded[0..8], KnownMagic::RainMetaDocumentV1.to_prefix_bytes());

        // first item in the encoded bytes
        // cbor map with 3 keys
        assert_eq!(cbor_encoded[8], 0xa3);
        // key 0
        assert_eq!(cbor_encoded[9], 0x00);
        // major type 2 (bytes) length 512
        assert_eq!(cbor_encoded[10], 0b010_11001);
        assert_eq!(cbor_encoded[11], 0b000_00010);
        assert_eq!(cbor_encoded[12], 0b000_00000);
        // payload
        assert_eq!(cbor_encoded[13..525], authoring_meta_abi_encoded);
        // key 1
        assert_eq!(cbor_encoded[525], 0x01);
        // major type 0 (unsigned integer) value 27
        assert_eq!(cbor_encoded[526], 0b000_11011);
        // magic number
        assert_eq!(&cbor_encoded[527..535], KnownMagic::AuthoringMetaV1.to_prefix_bytes());
        // key 2
        assert_eq!(cbor_encoded[535], 0x02);
        // text string application/cbor length 16
        assert_eq!(cbor_encoded[536], 0b011_10000);
        // the string application/cbor, must be the end of data
        assert_eq!(&cbor_encoded[537..553], "application/cbor".as_bytes());

        // second item in the encoded bytes
        // cbor map with 5 keys
        assert_eq!(cbor_encoded[553], 0xa5);
        // key 0
        assert_eq!(cbor_encoded[554], 0x00);
        // major type 2 (bytes) length 36
        assert_eq!(cbor_encoded[555], 0b010_11000);
        assert_eq!(cbor_encoded[556], 0b001_00100);
        // assert_eq!(cbor_encoded[4], 0b000_00000);
        // payload
        assert_eq!(cbor_encoded[557..593], deflated_payload);
        // key 1
        assert_eq!(cbor_encoded[593], 0x01);
        // major type 0 (unsigned integer) value 27
        assert_eq!(cbor_encoded[594], 0b000_11011);
        // magic number
        assert_eq!(&cbor_encoded[595..603], KnownMagic::DotrainV1.to_prefix_bytes());
        // key 2
        assert_eq!(cbor_encoded[603], 0x02);
        // text string application/octet-stream length 24
        assert_eq!(cbor_encoded[604], 0b011_11000);
        assert_eq!(cbor_encoded[605], 0b000_11000);
        // the string application/octet-stream
        assert_eq!(&cbor_encoded[606..630], "application/octet-stream".as_bytes());
        // key 3
        assert_eq!(cbor_encoded[630], 0x03);
        // text string deflate length 7
        assert_eq!(cbor_encoded[631], 0b011_00111);
        // the string deflate
        assert_eq!(&cbor_encoded[632..639], "deflate".as_bytes());
        // key 4
        assert_eq!(cbor_encoded[639], 0x04);
        // text string en length 2
        assert_eq!(cbor_encoded[640], 0b011_00010);
        // the string identity, must be the end of data
        assert_eq!(&cbor_encoded[641..], "en".as_bytes());

        // decode the data back to MetaMap
        let cbor_decoded = MetaMap::cbor_decode(&cbor_encoded)?;
        // the length of decoded maps must be 2 as we had 2 encoded item
        assert_eq!(cbor_decoded.len(), 2usize);

        // decoded item 1 must be equal to the original meta_map_1
        assert_eq!(cbor_decoded[0], meta_map_1);
        // decoded item 2 must be equal to the original meta_map_2
        assert_eq!(cbor_decoded[1], meta_map_2);

        // unpack the payload of first decoded map into AuthoringMeta
        let unpacked_payload_1: AuthoringMeta = cbor_decoded[0].unpack_into()?;
        // must be equal to original meta
        assert_eq!(unpacked_payload_1, authoring_meta);

        // unpack the payload of the second decoded map into DotrainMeta, should handle inflation of the payload internally
        let unpacked_payload_2: DotrainMeta = cbor_decoded[1].unpack_into()?;
        // must be equal to the original dotrain content
        assert_eq!(unpacked_payload_2, dotrain_content);

        Ok(())
    }
}