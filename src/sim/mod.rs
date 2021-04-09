//! Binary specific structures.

#[cfg(feature = "netcdf")]
pub mod babbage;

#[cfg(feature = "netcdf")]
pub mod cartographer;

#[cfg(feature = "netcdf")]
pub mod diffuse;

pub mod flask;

#[cfg(feature = "netcdf")]
pub mod reactor;

#[cfg(feature = "netcdf")]
#[cfg(feature = "png")]
pub mod mcrt;

pub mod powder;

#[cfg(feature = "png")]
pub mod render;
