use super::KnownMagic;
use strum::IntoEnumIterator;
use clap::Parser;

pub fn ls() -> anyhow::Result<()> {
    for magic in KnownMagic::iter() {
        println!("{:#x} {}", magic as u64, magic.to_string());
    }
    Ok(())
}