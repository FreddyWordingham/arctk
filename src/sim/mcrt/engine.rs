//! Engine function alias.

use crate::{
    phys::Photon,
    sim::mcrt::{Input, Output},
};
use rand::rngs::ThreadRng;

/// MCRT sampling engine function type.
pub type Engine = fn(input: &Input, &mut ThreadRng, phot: Photon, data: &mut Output);
