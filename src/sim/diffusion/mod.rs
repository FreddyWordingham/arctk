//! Diffusion simulation module.

pub mod input;
pub mod run;
pub mod stencil;

pub use self::{input::*, run::*, stencil::*};
