//! Engine function alias.

use crate::sim::mcrt::{Input, Output, Photon};
use rand::rngs::ThreadRng;

/// MCRT sampling engine function type.
pub type Engine = fn(input: &Input, &mut ThreadRng, phot: Photon, data: &mut Output);
