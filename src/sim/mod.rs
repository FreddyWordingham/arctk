//! Simulation specific structures.

#[cfg(feature = "netcdf")]
pub mod babbage;

#[cfg(feature = "netcdf")]
pub use self::babbage::*;
