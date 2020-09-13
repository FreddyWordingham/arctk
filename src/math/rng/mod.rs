//! Random-Number-Generation module.

pub mod distribution;
pub mod probability;
pub mod probability_builder;

pub use self::{distribution::*, probability::*, probability_builder::*};
