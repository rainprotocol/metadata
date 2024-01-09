use strum::IntoEnumIterator;
use crate::meta::magic::KnownMagic;

pub fn ls() -> anyhow::Result<()> {
    for magic in KnownMagic::iter() {
        println!("{:#x} {}", magic as u64, magic);
    }
    Ok(())
}
