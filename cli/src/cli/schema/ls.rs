use super::KnownSchema;
use strum::IntoEnumIterator;

pub fn ls() -> anyhow::Result<()> {
    for schema in KnownSchema::iter() {
        println!("{}", schema);
    }
    Ok(())
}