//! Photon-lifetime engine function.

use crate::mcrt::{Data, Photon, Sample, Scene};
use rand::rngs::ThreadRng;

/// Simulate the life of a within the photon.
#[inline]
#[must_use]
pub fn simulate_photon(
    scene: &Scene,
    _rng: &mut ThreadRng,
    data: &mut Data,
    phot: Photon,
) -> Sample {
    // Check photon is within the grid.
    if let Some(index) = scene.grid.gen_index(phot.ray().pos()) {
        data.emission_power[index] += phot.power() * phot.weight();
    } else {
        panic!("Photon was not emitted within the grid.");
    }

    // Common constants.
    let _bump_dist = scene.sett.bump_dist();
    let _loop_limit = scene.sett.loop_limit();

    // Initialisation.
    let mat = &scene.mats.map()["air"];
    let _env = mat.env(phot.wavelength());

    Sample::new(0.0)
}
