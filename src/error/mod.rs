//! Error enumeration.

use std::fmt::{Debug, Formatter};

/// Error enumeration.
pub enum Error {
    /// Integer parsing error.
    ParseInt(std::num::ParseIntError),
    /// Float parsing error.
    ParseFloat(std::num::ParseFloatError),
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

impl_from_for_err!(Self::ParseInt, std::num::ParseIntError);
impl_from_for_err!(Self::ParseFloat, std::num::ParseFloatError);

impl Debug for Error {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "{} error: `{}`",
            match self {
                Self::ParseInt { .. } => "Integer parsing",
                Self::ParseFloat { .. } => "Float parsing",
            },
            match self {
                Self::ParseInt { 0: err } => format!("{:?}", err),
                Self::ParseFloat { 0: err } => format!("{:?}", err),
            }
        )
    }
}
