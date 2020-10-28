//! Json loading.

use crate::err::Error;
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};

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

/// Deserialise the type in json format.
/// # Errors
/// if string can not be serialised into an instance of the required type.
#[inline]
pub fn from_json_str<T>(s: &str) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    Ok(json5::from_str(&s)?)
}
