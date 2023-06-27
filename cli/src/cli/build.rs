use crate::cli::output::SupportedOutputEncoding;
use crate::meta::magic::KnownMagic;
use crate::meta::normalize::normalize;
use crate::meta::ContentEncoding;
use crate::meta::ContentLanguage;
use crate::meta::ContentType;
use crate::meta::KnownMeta;
use crate::meta::RainMetaDocumentV1Item;
use anyhow::anyhow;
use clap::Parser;
use itertools::izip;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Build {
    #[arg(short, long)]
    output_path: Option<PathBuf>,
    #[arg(short = 'E', long, default_value = "binary")]
    output_encoding: SupportedOutputEncoding,
    #[arg(short = 'M', long, default_value = "rain-meta-document-v1")]
    global_magic: KnownMagic,
    #[arg(short, long, num_args = 1..)]
    input_path: Vec<PathBuf>,
    #[arg(short, long, num_args = 1..)]
    magic: Vec<KnownMagic>,
    #[arg(short = 't', long, num_args = 1..)]
    content_type: Vec<ContentType>,
    #[arg(short = 'e', long, num_args = 1..)]
    content_encoding: Vec<ContentEncoding>,
    #[arg(short = 'l', long, num_args = 1..)]
    content_language: Vec<ContentLanguage>,
}

pub struct BuildItem {
    pub data: Vec<u8>,
    pub magic: KnownMagic,
    pub content_type: ContentType,
    pub content_encoding: ContentEncoding,
    pub content_language: ContentLanguage,
}

impl TryFrom<&BuildItem> for RainMetaDocumentV1Item {
    type Error = anyhow::Error;
    fn try_from(item: &BuildItem) -> anyhow::Result<Self> {
        let normalized = match &item.magic {
            KnownMagic::SolidityAbiV2 => normalize(KnownMeta::SolidityAbiV2, &item.data)?,
            KnownMagic::OpMetaV1 => normalize(KnownMeta::OpV1, &item.data)?,
            KnownMagic::InterpreterCallerMetaV1 => {
                normalize(KnownMeta::InterpreterCallerMetaV1, &item.data)?
            }
            _ => return Err(anyhow!("Unsupported magic {}", item.magic)),
        };

        let encoded = match item.content_encoding {
            ContentEncoding::Deflate => deflate::deflate_bytes(&normalized),
            ContentEncoding::Identity | ContentEncoding::None => normalized,
        };

        Ok(RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(encoded),
            magic: item.magic,
            content_type: item.content_type,
            content_encoding: item.content_encoding,
            content_language: item.content_language,
        })
    }
}

impl BuildItem {
    fn write<W: std::io::Write>(&self, writer: &mut W) -> anyhow::Result<()> {
        Ok(ciborium::into_writer(&RainMetaDocumentV1Item::try_from(self)?, writer)?)
    }
}

fn build_bytes(magic: KnownMagic, items: Vec<BuildItem>) -> anyhow::Result<Vec<u8>> {
    let mut bytes: Vec<u8> = magic.to_prefix_bytes().to_vec();

    for item in items {
        item.write(&mut bytes)?;
    }
    Ok(bytes)
}

pub fn build(b: Build) -> anyhow::Result<()> {
    if b.input_path.len() != b.magic.len() {
        return Err(anyhow!(
            "{} inputs does not match {} magic numbers.",
            b.input_path.len(),
            b.magic.len()
        ));
    }

    if b.input_path.len() != b.content_type.len() {
        return Err(anyhow!(
            "{} inputs does not match {} content types.",
            b.input_path.len(),
            b.content_type.len()
        ));
    }

    if b.input_path.len() != b.content_encoding.len() {
        return Err(anyhow!(
            "{} inputs does not match {} content encodings.",
            b.input_path.len(),
            b.content_encoding.len()
        ));
    }

    if b.input_path.len() != b.content_language.len() {
        return Err(anyhow!(
            "{} inputs does not match {} content languages.",
            b.input_path.len(),
            b.content_language.len()
        ));
    }
    let items: Vec<BuildItem> = vec![];
    for(input_path, magic, content_type, content_encoding, content_language) in izip!(
        b.input_path.iter(),
        b.magic.iter(),
        b.content_type.iter(),
        b.content_encoding.iter(),
        b.content_language.iter()
    ) {
        items.push(BuildItem {
            data: std::fs::read(input_path)?,
            magic: *magic,
            content_type: *content_type,
            content_encoding: *content_encoding,
            content_language: *content_language,
        });
    }
    crate::cli::output::output(&b.output_path, b.output_encoding, &build_bytes(b.global_magic, items)?)
}

#[cfg(test)]
mod tests {


    use strum::IntoEnumIterator;
    use crate::{cli::output::SupportedOutputEncoding, meta::{magic::{self, KnownMagic}, ContentType, ContentEncoding, ContentLanguage, RainMetaDocumentV1Item}};
    use super::BuildItem;
    use super::{Build, build_bytes};

    #[test]
    fn test_build_empty() {
        for global_magic in magic::KnownMagic::iter() {
            let build: Build = Build {
                output_path: None,
                output_encoding: SupportedOutputEncoding::Hex,
                global_magic,
                input_path: vec![],
                magic: vec![],
                content_type: vec![],
                content_encoding: vec![],
                content_language: vec![],
            };

            let built_bytes = build_bytes(&build).unwrap();
            assert_eq!(built_bytes, global_magic.to_prefix_bytes());
        }
    }

    #[test]
    fn test_into_meta_document() -> anyhow::Result<()> {
        let build_item = BuildItem {
            data: "[]".as_bytes().to_vec(),
            magic: KnownMagic::SolidityAbiV2,
            content_type: ContentType::Json,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::En,
        };

        let meta_document = RainMetaDocumentV1Item::try_from(&build_item)?;
        let expected_meta_document = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from("[]".as_bytes().to_vec()),
            magic: KnownMagic::SolidityAbiV2,
            content_type: ContentType::Json,
            content_encoding: ContentEncoding::None,
            content_language: ContentLanguage::En,
        };
        assert_eq!(meta_document, expected_meta_document);
        Ok(())
    }

    // #[test]
    // fn test_build() {
    //     let build_struct: Build = Build {
    //         output_path: None,
    //         output_encoding: SupportedOutputEncoding::Hex,
    //         global_magic: magic::KnownMagic::SolidityAbiV2,
    //         input_path: vec!["test_abi.json".to_string().into()],
    //         magic: vec![KnownMagic::SolidityAbiV2],
    //         content_type: vec![ContentType::Json],
    //         content_encoding: vec![ContentEncoding::Deflate],
    //         content_language: vec![ContentLanguage::En],
    //     };

    //     let build_ = build_bytes(&build_struct).unwrap();
    //     let build_ = hex::encode(build_);
    //     let magic_number =  &build_[..16];
    //     let expected_magic_number = hex::encode(KnownMagic::RainMetaDocumentV1.to_prefix_bytes());

    //     let payload:&str = &build_[17..];
    //     let reader = Cursor::new(payload);
    //     // let decoded: RainMetaDocumentV1Item = from_reader(reader).expect("Fail to de-serialise");
    //     assert_eq!(magic_number, expected_magic_number);
    // }
}
