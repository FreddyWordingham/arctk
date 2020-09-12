//! Error handling.

use std::fmt::{Debug, Formatter};

/// Error enumeration.
pub enum Error {
    /// Description error.
    Text(String),
    /// Formatting error.
    Format(std::fmt::Error),
    /// Missing environment variable error.
    EnvVar(std::env::VarError),
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
    /// Shape error.
    InvalidShape(ndarray::ShapeError),
    /// NetCDF io error.
    #[cfg(feature = "netcdf")]
    NetCDF(netcdf::error::Error),
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

impl From<&str> for Error {
    #[inline]
    fn from(err: &str) -> Self {
        Self::Text(err.to_string())
    }
}

impl_from_for_err!(Self::Format, std::fmt::Error);
impl_from_for_err!(Self::EnvVar, std::env::VarError);
impl_from_for_err!(Self::LoadFile, std::io::Error);
impl_from_for_err!(Self::ParseInt, std::num::ParseIntError);
impl_from_for_err!(Self::ParseFloat, std::num::ParseFloatError);
impl_from_for_err!(Self::ReadJson, json5::Error);
impl_from_for_err!(Self::WriteJson, serde_json::Error);
impl_from_for_err!(Self::InvalidShape, ndarray::ShapeError);
#[cfg(feature = "netcdf")]
impl_from_for_err!(Self::NetCDF, netcdf::error::Error);

impl Debug for Error {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{} error: `{}`",
            match self {
                Self::Text { .. } => "Text string",
                Self::Format { .. } => "Formatting",
                Self::EnvVar { .. } => "Environment variable",
                Self::LoadFile { .. } => "File loading",
                Self::ParseInt { .. } => "Integer parsing",
                Self::ParseFloat { .. } => "Float parsing",
                Self::ReadJson { .. } => "Json reading",
                Self::WriteJson { .. } => "Json writing",
                Self::InvalidShape { .. } => "Invalid array shape",
                #[cfg(feature = "netcdf")]
                Self::NetCDF { .. } => "NetCDF IO",
            },
            match self {
                Self::Format { 0: err } => format!("{:?}", err),
                Self::Text { 0: err } => format!("{:?}", err),
                Self::EnvVar { 0: err } => format!("{:?}", err),
                Self::LoadFile { 0: err } => format!("{:?}", err),
                Self::ParseInt { 0: err } => format!("{:?}", err),
                Self::ParseFloat { 0: err } => format!("{:?}", err),
                Self::ReadJson { 0: err } => format!("{:?}", err),
                Self::WriteJson { 0: err } => format!("{:?}", err),
                Self::InvalidShape { 0: err } => format!("{:?}", err),
                #[cfg(feature = "netcdf")]
                Self::NetCDF { 0: err } => format!("{:?}", err),
            }
        )
    }
}
