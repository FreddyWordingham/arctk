//! File IO.

pub mod build;
pub mod json;
pub mod load;
pub mod redirect;
pub mod save;

#[cfg(feature = "netcdf")]
pub mod netcdf;

#[cfg(feature = "netcdf")]
pub use self::netcdf::*;

pub use self::{build::*, json::*, load::*, redirect::*, save::*};
