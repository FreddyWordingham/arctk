//! File extension loaders.

pub mod csv;
pub mod json;
pub mod netcdf;
pub mod png;
pub mod wavefront;

pub use self::{csv::*, json::*, netcdf::*, png::*, wavefront::*};
