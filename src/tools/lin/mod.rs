//! Linear tools module.

pub mod binner;
pub mod index;
pub mod range;
pub mod smooth;

pub use self::{binner::*, range::*, smooth::*};
