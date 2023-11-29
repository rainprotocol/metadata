pub mod magic;
pub mod types;
pub mod query;
pub mod normalize;

use reqwest::Client;
use futures::future;
use magic::KnownMagic;
use graphql_client::GraphQLQuery;
use strum::{EnumIter, EnumString};
use alloy_primitives::{keccak256, hex};
use serde::de::{Deserialize, Deserializer, Visitor};
use serde::ser::{Serialize, Serializer, SerializeMap};
use std::{sync::Arc, fmt::Debug, convert::TryFrom, collections::HashMap};

pub use super::subgraph::KnownSubgraphs;
pub use query::{MetaResponse, DeployerMetaResponse};

/// # Known Meta
/// all known meta identifiers
#[derive(Copy, Clone, EnumString, EnumIter, strum::Display, Debug, PartialEq)]
#[strum(serialize_all = "kebab-case")]
pub enum KnownMeta {
    OpV1,
    DotrainV1,
    RainlangV1,
    SolidityAbiV2,
    AuthoringMetaV1,
    InterpreterCallerMetaV1,
    ExpressionDeployerV2BytecodeV1,
}

impl TryFrom<KnownMagic> for KnownMeta {
    type Error = anyhow::Error;
    fn try_from(magic: KnownMagic) -> anyhow::Result<Self> {
        match magic {
            KnownMagic::OpMetaV1 => Ok(KnownMeta::OpV1),
            KnownMagic::DotrainV1 => Ok(KnownMeta::DotrainV1),
            KnownMagic::RainlangV1 => Ok(KnownMeta::RainlangV1),
            KnownMagic::SolidityAbiV2 => Ok(KnownMeta::SolidityAbiV2),
            KnownMagic::AuthoringMetaV1 => Ok(KnownMeta::AuthoringMetaV1),
            KnownMagic::InterpreterCallerMetaV1 => Ok(KnownMeta::InterpreterCallerMetaV1),
            KnownMagic::ExpressionDeployerV2BytecodeV1 => {
                Ok(KnownMeta::ExpressionDeployerV2BytecodeV1)
            }
            _ => Err(anyhow::anyhow!("Unsupported meta {}", magic)),
        }
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    EnumIter,
    PartialEq,
    EnumString,
    strum::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[strum(serialize_all = "kebab-case")]
pub enum ContentType {
    None,
    #[serde(rename = "application/json")]
    Json,
    #[serde(rename = "application/cbor")]
    Cbor,
    #[serde(rename = "application/octet-stream")]
    OctetStream,
}

#[derive(
    Copy,
    Clone,
    Debug,
    EnumIter,
    PartialEq,
    EnumString,
    strum::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ContentEncoding {
    None,
    Identity,
    Deflate,
}

impl ContentEncoding {
    /// encode the data based on the variant
    pub fn encode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            ContentEncoding::None | ContentEncoding::Identity => data.to_vec(),
            ContentEncoding::Deflate => deflate::deflate_bytes_zlib(data),
        })
    }

    /// decode the data based on the variant
    pub fn decode(&self, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        Ok(match self {
            ContentEncoding::None | ContentEncoding::Identity => data.to_vec(),
            ContentEncoding::Deflate => match inflate::inflate_bytes(data) {
                Ok(v) => v,
                Err(_) => {
                    match inflate::inflate_bytes_zlib(data) {
                        Ok(v) => v,
                        Err(error) => {
                            Err(anyhow::anyhow!(error))?
                        }
                    }
                }
            },
        })
    }
}

#[derive(
    Copy,
    Clone,
    Debug,
    EnumIter,
    PartialEq,
    EnumString,
    strum::Display,
    serde::Serialize,
    serde::Deserialize,
)]
#[serde(rename_all = "kebab-case")]
#[strum(serialize_all = "kebab-case")]
pub enum ContentLanguage {
    None,
    En,
}

/// # Rain Meta Document v1 Item (meta map)
/// represents a rain meta data and configuration that can be cbor encoded or unpacked back to the meta types
#[derive(PartialEq, Debug, Clone)]
pub struct RainMetaDocumentV1Item {
    pub payload: serde_bytes::ByteBuf,
    pub magic: KnownMagic,
    pub content_type: ContentType,
    pub content_encoding: ContentEncoding,
    pub content_language: ContentLanguage,
}

// this implementation is mainly used by Rainlang and Dotrain metas as they are aliased type for String
impl TryFrom<RainMetaDocumentV1Item> for String {
    type Error = anyhow::Error;
    fn try_from(value: RainMetaDocumentV1Item) -> Result<Self, Self::Error> {
        String::from_utf8(value.unpack()?).map_err(anyhow::Error::from)
    }
}

// this implementation is mainly used by ExpressionDeployerV2Bytecode meta as it is aliased type for Vec<u8>
impl TryFrom<RainMetaDocumentV1Item> for Vec<u8> {
    type Error = anyhow::Error;
    fn try_from(value: RainMetaDocumentV1Item) -> Result<Self, Self::Error> {
        value.unpack()
    }
}

impl RainMetaDocumentV1Item {
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
            Ok(keccak256(Self::cbor_encode_seq(
                &vec![self.clone()],
                KnownMagic::RainMetaDocumentV1,
            )?)
            .0)
        } else {
            Ok(keccak256(self.cbor_encode()?).0)
        }
    }

    /// method to cbor encode
    pub fn cbor_encode(&self) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = vec![];
        match serde_cbor::to_writer(&mut bytes, &self) {
            Ok(()) => Ok(bytes),
            Err(error) => Err(error)?,
        }
    }

    /// builds a cbor sequence from given MetaMaps
    pub fn cbor_encode_seq(
        seq: &Vec<RainMetaDocumentV1Item>,
        magic: KnownMagic,
    ) -> anyhow::Result<Vec<u8>> {
        let mut bytes: Vec<u8> = magic.to_prefix_bytes().to_vec();
        for item in seq {
            serde_cbor::to_writer(&mut bytes, &item)?;
        }
        Ok(bytes)
    }

    /// method to cbor decode from given bytes
    pub fn cbor_decode(data: &[u8]) -> anyhow::Result<Vec<RainMetaDocumentV1Item>> {
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
                match serde_cbor::value::from_value(cbor_map) {
                    Ok(meta) => metas.push(meta),
                    Err(error) => Err(error)?,
                };
                true
            }
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
            }
        } {}

        if metas.len() == 0
            || track.len() == 0
            || track.len() != metas.len()
            || len as usize != track[track.len() - 1]
        {
            return Err(anyhow::anyhow!("corrupt meta"));
        }
        Ok(metas)
    }

    // unpack the payload based on the configuration
    pub fn unpack(&self) -> anyhow::Result<Vec<u8>> {
        ContentEncoding::decode(&self.content_encoding, &self.payload.to_vec())
    }

    // unpacks the payload to given meta type based on configuration
    pub fn unpack_into<T: TryFrom<Self, Error = anyhow::Error>>(self) -> anyhow::Result<T> {
        // let data = self.unpack()?;
        match self.magic {
            KnownMagic::OpMetaV1
            | KnownMagic::DotrainV1
            | KnownMagic::RainlangV1
            | KnownMagic::SolidityAbiV2
            | KnownMagic::AuthoringMetaV1
            | KnownMagic::InterpreterCallerMetaV1
            | KnownMagic::ExpressionDeployerV2BytecodeV1 => {
                T::try_from(self).map_err(anyhow::Error::from)
            }
            _ => Err(anyhow::anyhow!("unsupproted magic number")),
        }
    }
}

impl Serialize for RainMetaDocumentV1Item {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len()))?;
        map.serialize_entry(&0, &self.payload)?;
        map.serialize_entry(&1, &(self.magic as u64))?;
        match self.content_type {
            ContentType::None => {}
            content_type => map.serialize_entry(&2, &content_type)?,
        }
        match self.content_encoding {
            ContentEncoding::None => {}
            content_encoding => map.serialize_entry(&3, &content_encoding)?,
        }
        match self.content_language {
            ContentLanguage::None => {}
            content_language => map.serialize_entry(&4, &content_language)?,
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for RainMetaDocumentV1Item {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct EncodedMap;
        impl<'de> Visitor<'de> for EncodedMap {
            type Value = RainMetaDocumentV1Item;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("rain meta cbor encoded bytes")
            }

            fn visit_map<T: serde::de::MapAccess<'de>>(
                self,
                mut map: T,
            ) -> Result<Self::Value, T::Error> {
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
                            other => Err(serde::de::Error::custom(&format!(
                                "found unexpected key in the map: {other}"
                            )))?,
                        };
                        true
                    }
                    Ok(None) => false,
                    Err(error) => Err(error)?,
                } {}
                let payload = payload.ok_or_else(|| serde::de::Error::missing_field("payload"))?;
                let magic = match magic
                    .ok_or_else(|| serde::de::Error::missing_field("magic number"))?
                    .try_into()
                {
                    Ok(m) => m,
                    _ => Err(serde::de::Error::custom("unknown magic number"))?,
                };
                let content_type = content_type.or(Some(ContentType::None)).unwrap();
                let content_encoding = content_encoding.or(Some(ContentEncoding::None)).unwrap();
                let content_language = content_language.or(Some(ContentLanguage::None)).unwrap();

                Ok(RainMetaDocumentV1Item {
                    payload,
                    magic,
                    content_type,
                    content_encoding,
                    content_language,
                })
            }
        }
        deserializer.deserialize_map(EncodedMap)
    }
}

/// # Search Meta
/// searches for a meta matching the given hash in given subgraphs urls
pub async fn search(
    hash: &str,
    subgraphs: &Vec<String>,
    timeout: u32,
) -> anyhow::Result<query::MetaResponse> {
    if !types::common::v1::HASH_PATTERN.is_match(hash) {
        return Err(anyhow::anyhow!("invalid hash"));
    }
    let request_body = query::MetaQuery::build_query(query::meta_query::Variables {
        hash: Some(hash.to_ascii_lowercase()),
    });
    let mut promises = vec![];
    let client = Arc::new(
        Client::builder()
            .timeout(std::time::Duration::from_secs(timeout as u64))
            .build()?,
    );
    for sg in subgraphs {
        promises.push(Box::pin(query::process_meta_query(
            client.clone(),
            &request_body,
            sg,
        )));
    }
    let response_value = future::select_ok(promises.drain(..)).await?.0;
    Ok(query::MetaResponse {
        bytes: hex::decode(response_value)?,
    })
}

/// # Search Deployer Meta
/// searches for a deployer meta matching the given hash in given subgraphs urls
pub async fn search_deployer(
    hash: &str,
    subgraphs: &Vec<String>,
    timeout: u32,
) -> anyhow::Result<query::DeployerMetaResponse> {
    if !types::common::v1::HASH_PATTERN.is_match(hash) {
        return Err(anyhow::anyhow!("invalid hash"));
    }
    let request_body = query::DeployerQuery::build_query(query::deployer_query::Variables {
        hash: Some(hash.to_ascii_lowercase()),
    });
    let mut promises = vec![];
    let client = Arc::new(
        Client::builder()
            .timeout(std::time::Duration::from_secs(timeout as u64))
            .build()?,
    );
    for sg in subgraphs {
        promises.push(Box::pin(query::process_deployer_query(
            client.clone(),
            &request_body,
            sg,
        )));
    }
    let response_value = future::select_ok(promises.drain(..)).await?.0;
    Ok(query::DeployerMetaResponse {
        hash: response_value.0,
        bytes: hex::decode(response_value.1)?,
    })
}

/// # Meta Store(CAS)
///
/// Reads, stores and simply manages k/v pairs of meta hash and meta bytes and provides the functionalities
/// to easliy utilize them. a hash is a 32 bytes data in hex string format and will be stored as lower case.
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
/// // hash of a meta to search and store
/// let hash = "some-hash".to_string();
///
/// // updates the meta store with a new meta by searching through subgraphs
/// store.update(&hash);
///
/// // updates the meta store with a new meta hash and bytes
/// store.update_with(&hash, &vec![0u8, 1u8]);
///
/// // to get a record from store
/// let meta = store.get_meta(&hash);
///
/// // to get a authoring meta record from store
/// let am = store.get_authoring_meta(&hash);
///
/// // path to a .rain file
/// let dotrain_uri = "path/to/file.rain";
///
/// // reading the dotrain content as an example,
/// // Store is agnostic to dotrain contents it just maps the hash of the content to the given
/// // uri and puts it as a new meta into the meta cache, so obtaining and passing the correct
/// // content is up to the implementer
/// let dotrain_content = std::fs::read_to_string(&dotrain_uri).unwrap_or(String::new());
///
/// // updates the dotrain cache for a dotrain text and uri
/// let (new_hash, old_hash) = store.set_dotrain(&dotrain_content, &dotrain_uri.to_string(), false).unwrap();
///
/// // to get dotrain meta bytes given a uri
/// let dotrain_meta_bytes = store.get_dotrain_meta(&dotrain_uri.to_string());
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
            subgraphs: KnownSubgraphs::NP.map(|url| url.to_string()).to_vec(),
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
            authoring_cache: HashMap::new(),
        }
    }

    /// creates new instance of Store with given initial values
    /// it checks the validity of each item of the provided values and only stores those that are valid
    pub fn create(
        subgraphs: &Vec<String>,
        cache: &HashMap<String, Vec<u8>>,
        authoring_cache: &HashMap<String, Vec<u8>>,
        dotrain_cache: &HashMap<String, String>,
        include_rain_subgraphs: bool,
    ) -> Store {
        let mut store;
        if include_rain_subgraphs {
            store = Store::default();
        } else {
            store = Store::new();
        }
        store.add_subgraphs(&subgraphs);
        for (hash, bytes) in cache {
            store.update_with(hash, bytes);
        }
        for (hash, bytes) in authoring_cache {
            if types::common::v1::HASH_PATTERN.is_match(hash) {
                let _h = hash.to_ascii_lowercase();
                if !store.authoring_cache.contains_key(&_h) {
                    if hex::encode_prefixed(keccak256(bytes)) == _h {
                        store.authoring_cache.insert(_h, bytes.clone());
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
    pub fn subgraphs(&self) -> &Vec<String> {
        &self.subgraphs
    }

    /// add new subgraph endpoints
    pub fn add_subgraphs(&mut self, subgraphs: &Vec<String>) {
        for sg in subgraphs {
            if !self.subgraphs.contains(sg) {
                self.subgraphs.push(sg.to_string());
            }
        }
    }

    /// getter method for the whole meta cache
    pub fn cache(&self) -> &HashMap<String, Vec<u8>> {
        &self.cache
    }

    /// get the corresponding meta bytes of the given hash if it exists
    pub fn get_meta(&self, hash: &str) -> Option<&Vec<u8>> {
        self.cache.get(&hash.to_ascii_lowercase())
    }

    /// getter method for the whole authoring meta cache
    pub fn authoring_cache(&self) -> &HashMap<String, Vec<u8>> {
        &self.authoring_cache
    }

    /// get the corresponding authoring meta bytes of the given hash if it exists
    pub fn get_authoring_meta(&self, hash: &str) -> Option<&Vec<u8>> {
        self.authoring_cache.get(&hash.to_ascii_lowercase())
    }

    /// searches for authoring meta in the subgraphs given the deployer hash
    pub async fn search_authoring_meta(&mut self, deployer_hash: &str) -> Option<&Vec<u8>> {
        match search_deployer(deployer_hash, &self.subgraphs, 6u32).await {
            Ok(res) => self.update_with(&res.hash, &res.bytes),
            Err(_e) => None,
        }
    }

    /// if the authoring meta already is cached it returns it immediately else
    /// searches for authoring meta in the subgraphs given the deployer hash
    pub async fn search_authoring_meta_check(&mut self, authoring_meta_hash: &str, deployer_hash: &str) -> Option<&Vec<u8>> {
        if self
            .authoring_cache
            .contains_key(&authoring_meta_hash.to_ascii_lowercase())
        {
            self.get_authoring_meta(authoring_meta_hash)
        } else {
            self.search_authoring_meta(deployer_hash).await
        }
    }

    /// getter method for the whole dotrain cache
    pub fn dotrain_cache(&self) -> &HashMap<String, String> {
        &self.dotrain_cache
    }

    /// get the corresponding dotrain hash of the given dotrain uri if it exists
    pub fn get_dotrain_hash(&self, uri: &str) -> Option<&String> {
        self.dotrain_cache.get(uri)
    }

    /// get the corresponding uri of the given dotrain hash if it exists
    pub fn get_dotrain_uri(&self, hash: &str) -> Option<&str> {
        for (uri, h) in &self.dotrain_cache {
            if h.eq_ignore_ascii_case(hash) {
                return Some(uri);
            }
        }
        return None;
    }

    /// get the corresponding meta bytes of the given dotrain uri if it exists
    pub fn get_dotrain_meta(&self, uri: &str) -> Option<&Vec<u8>> {
        self.get_meta(self.dotrain_cache.get(uri)?)
    }

    /// deletes a dotrain record given a uri
    pub fn delete_dotrain(&mut self, uri: &str, keep_meta: bool) {
        if let Some(kv) = self.dotrain_cache.remove_entry(uri) {
            if !keep_meta {
                self.cache.remove(&kv.1);
            }
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
                self.authoring_cache
                    .insert(hash.to_ascii_lowercase(), bytes.clone());
            }
        }
        for (uri, hash) in &other.dotrain_cache {
            if !self.dotrain_cache.contains_key(uri) {
                self.dotrain_cache
                    .insert(uri.clone(), hash.to_ascii_lowercase());
            }
        }
    }

    /// updates the meta cache by searching through all subgraphs for the given hash
    /// returns the reference to the meta bytes in the cache if it was found
    pub async fn update(&mut self, hash: &str) -> Option<&Vec<u8>> {
        if let Ok(meta) = search(hash, &self.subgraphs, 6u32).await {
            self.store_content(&meta.bytes);
            self.cache.insert(hash.to_ascii_lowercase(), meta.bytes);
            return self.get_meta(hash);
        } else {
            return None;
        }
    }

    /// first checks if the meta is stored, if not will perform update()
    pub async fn update_check(&mut self, hash: &str) -> Option<&Vec<u8>> {
        if types::common::v1::HASH_PATTERN.is_match(hash) {
            let h = hash.to_ascii_lowercase();
            if !self.cache.contains_key(&h) {
                self.update(hash).await
            } else {
                return self.get_meta(hash);
            }
        } else {
            return None;
        }
    }

    /// updates the meta cache by the given hash and meta bytes, checks the hash to bytes 
    /// validity returns the reference to the bytes if the updated meta bytes contained any
    pub fn update_with(&mut self, hash: &str, bytes: &[u8]) -> Option<&Vec<u8>> {
        // let mut am_bytes: Option<&Vec<u8>> = None;
        if types::common::v1::HASH_PATTERN.is_match(hash) {
            let h = hash.to_ascii_lowercase();
            if !self.cache.contains_key(&h) {
                if hex::encode_prefixed(keccak256(bytes)) == h {
                    self.store_content(bytes);
                    self.cache.insert(h.clone(), bytes.to_vec());
                    return self.cache.get(&h);
                } else {
                    return None
                }
            } else {
                return self.get_meta(hash);
            }
        } else {
            return None;
        }
    }

    /// stores (or updates in case the URI already exists) the given dotrain text as meta into the store cache
    /// and maps it to the given uri (path), it should be noted that reading the content of the dotrain is not in
    /// the scope of Store and handling and passing on a correct URI (path) for the given text must be handled
    /// externally by the implementer
    pub fn set_dotrain(
        &mut self,
        text: &str,
        uri: &str,
        keep_old: bool,
    ) -> anyhow::Result<(String, String)> {
        let bytes = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(text.as_bytes()),
            magic: KnownMagic::DotrainV1,
            content_type: ContentType::OctetStream,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::None,
        }.cbor_encode()?;
        let new_hash = hex::encode_prefixed(keccak256(&bytes));
        if let Some(h) = self.dotrain_cache.get(uri) {
            let old_hash = h.clone();
            if new_hash.eq_ignore_ascii_case(&old_hash) {
                self.cache.insert(new_hash.clone(), bytes);
                return Ok((new_hash, String::new()));
            } else {
                self.cache.insert(new_hash.clone(), bytes);
                self.dotrain_cache.insert(uri.to_string(), new_hash.clone());
                if !keep_old {
                    self.cache.remove(&old_hash);
                }
                return Ok((new_hash, old_hash));
            }
        } else {
            self.dotrain_cache.insert(uri.to_string(), new_hash.clone());
            self.cache.insert(new_hash.clone(), bytes);
            return Ok((new_hash, String::new()));
        };
    }

    /// decodes each meta and stores the inner meta items into the cache
    /// if any of the inner items is an authoring meta, stores it in authoring meta cache as well
    /// returns the reference to the authoring bytes if the meta bytes contained any
    fn store_content(&mut self, bytes: &[u8]) {
        if let Ok(meta_maps) = RainMetaDocumentV1Item::cbor_decode(bytes) {
            if bytes.starts_with(&KnownMagic::RainMetaDocumentV1.to_prefix_bytes()) {
                for meta_map in &meta_maps {
                    if meta_map.magic == KnownMagic::AuthoringMetaV1 {
                        if let Ok(am_bytes) = meta_map.unpack() {
                            self.authoring_cache.insert(
                                hex::encode_prefixed(keccak256(&am_bytes)), 
                                am_bytes
                            );
                        }
                    }
                    if let Ok(encoded_bytes) = meta_map.cbor_encode() {
                        self.cache.insert(
                            hex::encode_prefixed(keccak256(&encoded_bytes)), 
                            encoded_bytes
                        );
                    }
                }
            } else {
                if meta_maps.len() == 1 && meta_maps[0].magic == KnownMagic::AuthoringMetaV1 {
                    if let Ok(am_bytes) = meta_maps[0].unpack() {
                        self.authoring_cache.insert(
                            hex::encode_prefixed(keccak256(&am_bytes)), 
                            am_bytes
                        );
                    }
                }
            }
        }
    }
}

/// converts string to bytes32
pub fn str_to_bytes32(text: &str) -> anyhow::Result<[u8; 32]> {
    let bytes: &[u8] = text.as_bytes();
    if bytes.len() > 32 {
        return Err(anyhow::anyhow!(
            "unexpected length, must be 32 bytes or less"
        ));
    }
    let mut b32 = [0u8; 32];
    b32[..bytes.len()].copy_from_slice(bytes);
    Ok(b32)
}

/// converts bytes32 to string
pub fn bytes32_to_str(bytes: &[u8; 32]) -> anyhow::Result<&str> {
    let mut len = 32;
    if let Some((pos, _)) = itertools::Itertools::find_position(&mut bytes.iter(), |b| **b == 0u8) {
        len = pos;
    };
    Ok(std::str::from_utf8(&bytes[..len])?)
}

#[cfg(test)]
mod tests {
    use alloy_primitives::hex;
    use alloy_sol_types::SolType;
    use super::{
        str_to_bytes32, bytes32_to_str,
        magic::KnownMagic,
        RainMetaDocumentV1Item, ContentType, ContentEncoding, ContentLanguage,
        types::{dotrain::v1::DotrainMeta, authoring::v1::AuthoringMeta},
    };

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
        let authoring_meta: AuthoringMeta = serde_json::from_str(authoring_meta_content)?;

        // abi encode the authoring meta with performing validation
        let authoring_meta_abi_encoded = authoring_meta.abi_encode_validate()?;
        let expected_abi_encoded =
            <alloy_sol_types::sol!((bytes32, uint8, string)[])>::abi_encode(&vec![
                (
                    str_to_bytes32("stack")?,
                    16u8,
                    "Copies an existing value from the stack.".to_string(),
                ),
                (
                    str_to_bytes32("constant")?,
                    16u8,
                    "Copies a constant value onto the stack.".to_string(),
                ),
            ]);
        // check the encoded bytes agaiinst the expected
        assert_eq!(authoring_meta_abi_encoded, expected_abi_encoded);

        let meta_map = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(authoring_meta_abi_encoded.clone()),
            magic: KnownMagic::AuthoringMetaV1,
            content_type: ContentType::Cbor,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::None,
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
        assert_eq!(
            &cbor_encoded[519..527],
            KnownMagic::AuthoringMetaV1.to_prefix_bytes()
        );
        // key 2
        assert_eq!(cbor_encoded[527], 0x02);
        // text string application/cbor length 16
        assert_eq!(cbor_encoded[528], 0b011_10000);
        // the string application/cbor, must be the end of data
        assert_eq!(&cbor_encoded[529..], "application/cbor".as_bytes());

        // decode the data back to MetaMap
        let mut cbor_decoded = RainMetaDocumentV1Item::cbor_decode(&cbor_encoded)?;
        // the length of decoded maps must be 1 as we only had 1 encoded item
        assert_eq!(cbor_decoded.len(), 1);
        // decoded item must be equal to the original meta_map
        assert_eq!(cbor_decoded[0], meta_map);

        // unpack the payload into AuthoringMeta
        let unpacked_payload: AuthoringMeta = cbor_decoded.pop().unwrap().unpack_into()?;
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

        let meta_map = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(deflated_payload.clone()),
            magic: KnownMagic::DotrainV1,
            content_type: ContentType::OctetStream,
            content_encoding,
            content_language: ContentLanguage::En,
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
        assert_eq!(
            &cbor_encoded[42..50],
            KnownMagic::DotrainV1.to_prefix_bytes()
        );
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
        let mut cbor_decoded = RainMetaDocumentV1Item::cbor_decode(&cbor_encoded)?;
        // the length of decoded maps must be 1 as we only had 1 encoded item
        assert_eq!(cbor_decoded.len(), 1);
        // decoded item must be equal to the original meta_map
        assert_eq!(cbor_decoded[0], meta_map);

        // unpack the payload into DotrainMeta, should handle inflation of the payload internally
        let unpacked_payload: DotrainMeta = cbor_decoded.pop().unwrap().unpack_into()?;
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
        let meta_map_1 = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(authoring_meta_abi_encoded.clone()),
            magic: KnownMagic::AuthoringMetaV1,
            content_type: ContentType::Cbor,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::None,
        };

        let dotrain_content = "#main _ _: int-add(1 2) int-add(2 3)";
        let dotrain_content_bytes = dotrain_content.as_bytes().to_vec();
        let content_encoding = ContentEncoding::Deflate;
        let deflated_payload = content_encoding.encode(&dotrain_content_bytes)?;
        let meta_map_2 = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(deflated_payload.clone()),
            magic: KnownMagic::DotrainV1,
            content_type: ContentType::OctetStream,
            content_encoding,
            content_language: ContentLanguage::En,
        };

        // cbor encode as RainMetaDocument sequence
        let cbor_encoded = RainMetaDocumentV1Item::cbor_encode_seq(
            &vec![meta_map_1.clone(), meta_map_2.clone()],
            KnownMagic::RainMetaDocumentV1,
        )?;

        // 8 byte magic number prefix
        assert_eq!(
            &cbor_encoded[0..8],
            KnownMagic::RainMetaDocumentV1.to_prefix_bytes()
        );

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
        assert_eq!(
            &cbor_encoded[527..535],
            KnownMagic::AuthoringMetaV1.to_prefix_bytes()
        );
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
        assert_eq!(
            &cbor_encoded[595..603],
            KnownMagic::DotrainV1.to_prefix_bytes()
        );
        // key 2
        assert_eq!(cbor_encoded[603], 0x02);
        // text string application/octet-stream length 24
        assert_eq!(cbor_encoded[604], 0b011_11000);
        assert_eq!(cbor_encoded[605], 0b000_11000);
        // the string application/octet-stream
        assert_eq!(
            &cbor_encoded[606..630],
            "application/octet-stream".as_bytes()
        );
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
        let mut cbor_decoded = RainMetaDocumentV1Item::cbor_decode(&cbor_encoded)?;
        // the length of decoded maps must be 2 as we had 2 encoded item
        assert_eq!(cbor_decoded.len(), 2);

        // decoded item 1 must be equal to the original meta_map_1
        assert_eq!(cbor_decoded[0], meta_map_1);
        // decoded item 2 must be equal to the original meta_map_2
        assert_eq!(cbor_decoded[1], meta_map_2);

        // unpack the payload of the second decoded map into DotrainMeta, should handle inflation of the payload internally
        let unpacked_payload_2: DotrainMeta = cbor_decoded.pop().unwrap().unpack_into()?;
        // must be equal to original meta
        assert_eq!(unpacked_payload_2, dotrain_content);

        // unpack the payload of first decoded map into AuthoringMeta
        let unpacked_payload_1: AuthoringMeta = cbor_decoded.pop().unwrap().unpack_into()?;
        // must be equal to the original dotrain content
        assert_eq!(unpacked_payload_1, authoring_meta);

        Ok(())
    }

    #[test]
    fn test_bytes32_to_str() {
        let text_bytes_list = vec![
            (
                "",
                hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "A",
                hex!("4100000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ012345",
                hex!("4142434445464748494a4b4c4d4e4f505152535455565758595a303132333435"),
            ),
            (
                "!@#$%^&*(),./;'[]",
                hex!("21402324255e262a28292c2e2f3b275b5d000000000000000000000000000000"),
            ),
        ];

        for (text, bytes) in text_bytes_list {
            assert_eq!(text, bytes32_to_str(&bytes).unwrap());
        }
    }

    #[test]
    fn test_str_to_bytes32() {
        let text_bytes_list = vec![
            (
                "",
                hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "A",
                hex!("4100000000000000000000000000000000000000000000000000000000000000"),
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ012345",
                hex!("4142434445464748494a4b4c4d4e4f505152535455565758595a303132333435"),
            ),
            (
                "!@#$%^&*(),./;'[]",
                hex!("21402324255e262a28292c2e2f3b275b5d000000000000000000000000000000"),
            ),
        ];

        for (text, bytes) in text_bytes_list {
            assert_eq!(bytes, str_to_bytes32(text).unwrap());
        }
    }

    #[test]
    fn test_str_to_bytes32_long() {
        assert!(matches!(
            str_to_bytes32("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456").unwrap_err(),
            anyhow::Error { .. }
        ));
    }
}
