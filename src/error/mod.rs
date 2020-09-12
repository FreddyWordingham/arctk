//! Error enumeration.

use std::fmt::{Debug, Formatter};

/// Error enumeration.
pub enum Error {
    /// File loading error.
    LoadFile(std::io::Error),
    /// Integer parsing error.
    ParseInt(std::num::ParseIntError),
    /// Float parsing error.
    ParseFloat(std::num::ParseFloatError),
    /// Json reading error.
    ReadJson(json5::Error),
    /// Json writing error.
    WriteJson(serde_json::Error),
}

macro_rules! impl_from_for_err {
    ($enum:path, $error:ty) => {
        impl From<$error> for Error {
            #[inline]
            fn from(e: $error) -> Self {
                $enum(e)
            }
        }
    };
}

impl_from_for_err!(Self::LoadFile, std::io::Error);
impl_from_for_err!(Self::ParseInt, std::num::ParseIntError);
impl_from_for_err!(Self::ParseFloat, std::num::ParseFloatError);
impl_from_for_err!(Self::ReadJson, json5::Error);
impl_from_for_err!(Self::WriteJson, serde_json::Error);

impl Debug for Error {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{} error: `{}`",
            match self {
                Self::LoadFile { .. } => "File loading",
                Self::ParseInt { .. } => "Integer parsing",
                Self::ParseFloat { .. } => "Float parsing",
                Self::ReadJson { .. } => "Json reading",
                Self::WriteJson { .. } => "Json writing",
            },
            match self {
                Self::LoadFile { 0: err } => format!("{:?}", err),
                Self::ParseInt { 0: err } => format!("{:?}", err),
                Self::ParseFloat { 0: err } => format!("{:?}", err),
                Self::ReadJson { 0: err } => format!("{:?}", err),
                Self::WriteJson { 0: err } => format!("{:?}", err),
            }
        )
    }
}
