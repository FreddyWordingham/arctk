//! Simulation specific structures.

#[cfg(feature = "netcdf")]
pub mod babbage;
pub mod cartograph;

#[cfg(feature = "netcdf")]
pub use self::{babbage::*, cartograph::*};
