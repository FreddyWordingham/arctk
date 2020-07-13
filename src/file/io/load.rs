//! Load trait.

use crate::Error;
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};

/// Types implementing this trait can be loaded from a file.
pub trait Load
where
    Self: std::marker::Sized,
{
    /// Deserialize the type from a given file.
    /// # Errors
    /// if the target file can not be found,
    /// or the read string can not be serialised into an instance of the required type.
    fn load(path: &Path) -> Result<Self, Error>;
}

/// Deserialise the type in json format.
/// # Errors
/// if file can not be opened or read string can not be serialised into an instance of the required type.
#[inline]
pub fn from_json<T>(path: &Path) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    println!("loading: {}", path.display());
    let s = read_to_string(path)?;
    Ok(json5::from_str(&s)?)
}
