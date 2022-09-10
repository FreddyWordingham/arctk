//! File format parsers.

pub mod json;
pub mod png;
pub mod wavefront;

pub use self::{json::*, png::*, wavefront::*};
