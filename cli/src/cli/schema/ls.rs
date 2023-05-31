use super::KnownSchema;
use strum::IntoEnumIterator;

pub fn ls() {
    for schema in KnownSchema::iter() {
        println!("{}", schema);
    }
}