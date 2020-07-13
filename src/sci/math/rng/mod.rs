//! Random-Number-Generation module.

pub mod distribution;
pub mod perlin_map;
pub mod probability;

pub use self::{perlin_map::*, probability::*};
