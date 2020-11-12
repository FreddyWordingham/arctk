//! File re-direction implementation.

use crate::{
    err::Error,
    file::{as_json, from_json, Build, Load, Save},
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

impl<T: Load> Load for Redirect<T>
where
    for<'de> T: Deserialize<'de>,
{
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        println!("loading: {}", path.display());
        from_json(path)
    }
}

impl<T: Serialize> Save for Redirect<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        as_json(self, path)
    }
}

impl<T: Load> Build for Redirect<T> {
    type Inst = T;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        match self {
            Self::There(path) => {
                let path = in_dir.join(path);
                T::load(&path)
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
