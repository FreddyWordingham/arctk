//! Error enumeration.

use std::fmt::{Debug, Formatter};

/// Error enumeration.
pub enum Error {
    /// File loading error.
    Load(std::io::Error),
    /// Reading error.
    Read(json5::Error),
    /// Hexadecimal parsing error.
    ParseHex(hex::FromHexError),
    /// Integer parsing error.
    ParseInt(std::num::ParseIntError),
    /// Float parsing error.
    ParseFloat(std::num::ParseFloatError),
    /// Json writing error.
    WriteJson(serde_json::Error),
    // /// MiniFB window error.
    // MiniFB(minifb::Error),
    /// PNG writing error.
    WritePng(png::EncodingError),
    /// Environment variable error.
    EnvVar(std::env::VarError),
    /// Formatting error.
    Format(std::fmt::Error),
    /// Shape error.
    Shape(ndarray::ShapeError),
    /// Min/max error.
    MinMax(ndarray_stats::errors::MinMaxError),
    /// Parallelisation poison.
    Parallel,
    /// String error.
    Text(String),
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

impl_from_for_err!(Self::Load, std::io::Error);
impl_from_for_err!(Self::Read, json5::Error);
impl_from_for_err!(Self::ParseHex, hex::FromHexError);
impl_from_for_err!(Self::ParseInt, std::num::ParseIntError);
impl_from_for_err!(Self::ParseFloat, std::num::ParseFloatError);
impl_from_for_err!(Self::WriteJson, serde_json::Error);
// impl_from_for_err!(Self::MiniFB, minifb::Error);
impl_from_for_err!(Self::WritePng, png::EncodingError);
impl_from_for_err!(Self::EnvVar, std::env::VarError);
impl_from_for_err!(Self::Format, std::fmt::Error);
impl_from_for_err!(Self::Shape, ndarray::ShapeError);
impl_from_for_err!(Self::MinMax, ndarray_stats::errors::MinMaxError);

impl<T> From<std::sync::PoisonError<T>> for Error {
    #[inline]
    fn from(_e: std::sync::PoisonError<T>) -> Self {
        Self::Parallel
    }
}

impl From<&str> for Error {
    #[inline]
    fn from(err: &str) -> Self {
        Self::Text(err.to_string())
    }
}

impl Debug for Error {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{} error: `{}`",
            match self {
                Self::Load { .. } => "Loading",
                Self::Read { .. } => "Reading",
                Self::ParseHex { .. } => "Hexadecimal parsing",
                Self::ParseInt { .. } => "Integer parsing",
                Self::ParseFloat { .. } => "Float parsing",
                Self::WriteJson { .. } => "JSON writing",
                // Self::MiniFB { .. } => "MiniFB window error",
                Self::WritePng { .. } => "PNG writing",
                Self::EnvVar { .. } => "Environment variable missing",
                Self::Format { .. } => "Formatting",
                Self::Shape { .. } => "Shape",
                Self::MinMax { .. } => "MinMax",
                Self::Parallel { .. } => "Parallelisation poison",
                Self::Text { .. } => "Text string",
            },
            match self {
                Self::Load { 0: err } => format!("{:?}", err),
                Self::Read { 0: err } => format!("{:?}", err),
                Self::ParseHex { 0: err } => format!("{:?}", err),
                Self::ParseInt { 0: err } => format!("{:?}", err),
                Self::ParseFloat { 0: err } => format!("{:?}", err),
                Self::WriteJson { 0: err } => format!("{:?}", err),
                // Self::MiniFB { 0: err } => format!("{:?}", err),
                Self::WritePng { 0: err } => format!("{:?}", err),
                Self::EnvVar { 0: err } => format!("{:?}", err),
                Self::Format { 0: err } => format!("{:?}", err),
                Self::Shape { 0: err } => format!("{:?}", err),
                Self::MinMax { 0: err } => format!("{:?}", err),
                Self::Parallel => "Parallelisation fail".to_string(),
                Self::Text { 0: err } => format!("{:?}", err),
            }
        )
    }
}
