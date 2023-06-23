use crate::meta::magic::KnownMagic;
use std::path::PathBuf;
use clap::Parser;
use anyhow::anyhow;
use crate::meta::normalize::normalize;
use crate::meta::KnownMeta;
use crate::meta::RainMetaDocumentV1Item;
use crate::meta::ContentType;
use crate::meta::ContentEncoding;
use crate::meta::ContentLanguage;
use itertools::izip;
use crate::cli::output::SupportedOutputEncoding;

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

fn build_bytes(build: &Build) -> anyhow::Result<Vec<u8>> {
    let mut bytess: Vec<Vec<u8>> = Vec::new();
    bytess.push(build.global_magic.to_prefix_bytes().to_vec());

    if build.input_path.len() != build.magic.len() {
        return Err(anyhow!("{} inputs does not match {} magic numbers.", build.input_path.len(), build.magic.len()));
    }

    if build.input_path.len() != build.content_type.len() {
        return Err(anyhow!("{} inputs does not match {} content types.", build.input_path.len(), build.content_type.len()));
    }

    if build.input_path.len() != build.content_encoding.len() {
        return Err(anyhow!("{} inputs does not match {} content encodings.", build.input_path.len(), build.content_encoding.len()));
    }

    if build.input_path.len() != build.content_language.len() {
        return Err(anyhow!("{} inputs does not match {} content languages.", build.input_path.len(), build.content_language.len()));
    }

    for (
        input_path,
        magic,
        content_type,
        content_encoding,
        content_language
    ) in izip!(
        build.input_path.iter(),
        build.magic.iter(),
        build.content_type.iter(),
        build.content_encoding.iter(),
        build.content_language.iter()) {
        bytess.push(magic.to_prefix_bytes().to_vec());

        let data = std::fs::read(input_path)?;
        let normalized = match magic {
            KnownMagic::SolidityAbiV2 => normalize(KnownMeta::SolidityAbiV2, &data)?,
            KnownMagic::OpMetaV1 => normalize(KnownMeta::OpV1, &data)?,
            KnownMagic::InterpreterCallerMetaV1 => normalize(KnownMeta::InterpreterCallerMetaV1, &data)?,
            _ => return Err(anyhow!("Unsupported magic {}", magic)),
        };

        let encoded = match content_encoding {
            ContentEncoding::Deflate => { deflate::deflate_bytes(&normalized) },
            ContentEncoding::Identity | ContentEncoding::None => { normalized },
        };

        let item = RainMetaDocumentV1Item {
            payload: serde_bytes::ByteBuf::from(encoded),
            magic: *magic,
            content_type: *content_type,
            content_encoding: *content_encoding,
            content_language: *content_language,
        };
        let mut cbor_data: Vec<u8> = Vec::new();
        ciborium::into_writer(&item, &mut cbor_data)?;
        bytess.push(cbor_data);
    }
    Ok(bytess.into_iter().flatten().collect())
}

pub fn build(b: Build) -> anyhow::Result<()> {
    crate::cli::output::output(&b.output_path, b.output_encoding, &build_bytes(&b)?)
}