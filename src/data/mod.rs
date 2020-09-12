//! Data reduction tools.

pub mod average;
pub mod binner;
pub mod histogram;
pub mod range;

pub use self::{average::*, histogram::*};
pub use self::{binner::*, range::*};
