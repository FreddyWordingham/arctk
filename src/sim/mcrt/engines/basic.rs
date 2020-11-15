//! Photon-lifetime engine function.

use crate::{
    opt::Photon,
    sim::mcrt::{Input, Output},
};
use rand::rngs::ThreadRng;

/// Simulate the life of a single photon.
#[inline]
#[must_use]
pub fn basic(_input: &Input, _rng: &mut ThreadRng, _phot: Photon, data: Output) -> Output {
    println!("BASIC ENGINE!");
    data
}
