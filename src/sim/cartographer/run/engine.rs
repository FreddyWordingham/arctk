//! Engine function alias.

use crate::sim::cartographer::{Input, Output};
use rand::rngs::ThreadRng;

/// Cartographer sampling engine function type.
pub type Engine = fn(input: &Input, &mut ThreadRng, index: [usize; 3], data: &mut Output);
