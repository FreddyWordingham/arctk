//! Diffusion stencils module.

pub mod grad;
pub mod reflect;

pub use self::{grad::*, reflect::*};
