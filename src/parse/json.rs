//! JavaScript Object Notation.

use core::any::type_name;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::{fs, path::Path};

/// Load a type from a JSON file.
#[inline]
#[must_use]
pub fn load<T>(path: &Path) -> T
where
    for<'de> T: Deserialize<'de>,
{
    read(
        &fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}.", path.display())),
    )
}

/// Read a type from a JSON string.
#[inline]
#[must_use]
pub fn read<T>(s: &str) -> T
where
    for<'de> T: Deserialize<'de>,
{
    json5::from_str(s).unwrap_or_else(|_| {
        panic!(
            "Failed to create type {} from JSON string.",
            type_name::<T>()
        )
    })
}

/// Serialise a type in json format.
#[inline]
pub fn save<T: Serialize>(instance: &T, path: &Path) {
    fs::write(path, write(instance))
        .unwrap_or_else(|_| panic!("Failed to write file: {}.", path.display()));
}

/// Serialise a type in json format.
#[inline]
#[must_use]
pub fn write<T: Serialize>(instance: &T) -> String {
    to_string(instance)
        .unwrap_or_else(|_| panic!("Failed to write type {} to JSON string.", type_name::<T>()))
}
