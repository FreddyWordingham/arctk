//! File re-direction implementation.

use crate::{
    err::Error,
    fs::{as_json, from_json, File, Load, Save},
};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Possible file redirection structure.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Redirect<T> {
    /// Path to file.
    There(String),
    /// Direct value.
    Here(T),
}

impl<T: File> File for Redirect<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        from_json(path)
    }
}

impl<T: Serialize> Save for Redirect<T> {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
        as_json(self, path)
    }
}

impl<T: File> Load for Redirect<T> {
    type Inst = T;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        match self {
            Self::There(path) => {
                let path = in_dir.join(path);
                T::new_from_file(&path)
            }
            Self::Here(val) => Ok(val),
        }
    }
}

impl<T: Display> Display for Redirect<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        match *self {
            Self::There(ref path) => write!(fmt, "-> {}", path),
            Self::Here(ref item) => write!(fmt, "_! {}", item),
        }
    }
}
