//! Photon-lifetime engine function.

use crate::{
    opt::Photon,
    sim::mcrt::{Input, Output},
};
use rand::rngs::ThreadRng;

/// Simulate the life of a single photon.
#[inline]
#[must_use]
pub fn basic(input: &Input, _rng: &mut ThreadRng, phot: Photon, mut data: Output) -> Output {
    // Check photon is within the grid.
    if let Some(index) = input.grid.gen_index(phot.ray().pos()) {
        data.emission[index] += phot.power() * phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Common constants.
    let bump_dist = input.sett.bump_dist();

    // Main event loop.

    println!("BASIC ENGINE!");
    data
}
