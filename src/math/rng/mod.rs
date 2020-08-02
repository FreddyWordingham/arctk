//! Random-Number-Generation module.

pub mod distribution;
pub mod perlin_map;
pub mod probability;
pub mod probability_form;

pub use self::{perlin_map::*, probability::*, probability_form::*};
