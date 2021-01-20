//! Binary specific structures.

#[cfg(feature = "netcdf")]
pub mod babbage;

#[cfg(feature = "netcdf")]
pub mod cartographer;

pub mod flask;

#[cfg(feature = "netcdf")]
pub mod mcrt;

#[cfg(feature = "png")]
pub mod render;
