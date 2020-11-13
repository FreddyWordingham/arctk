//! Engine functions.

use super::{Data, Photon, Sample, Universe};
use rand::rngs::ThreadRng;

/// MCRT sampling engine function type.
pub type Engine = fn(&mut ThreadRng, uni: &Universe, data: &mut Data, phot: Photon) -> Sample;

pub mod basic;
