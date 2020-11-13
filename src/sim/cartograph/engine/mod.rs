//! Engine functions.

use super::{Data, Landscape};
use rand::rngs::ThreadRng;

/// Cartography sampling engine function type.
pub type Engine = fn(land: &Landscape, data: &mut Data, index: [usize; 3], &mut ThreadRng);

pub mod basic;
