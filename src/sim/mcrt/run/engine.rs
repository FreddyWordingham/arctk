//! Photon-lifetime engine function.

use crate::mcrt::{Light, Photon, Sample, Scene};
use rand::{rngs::ThreadRng, Rng};

/// Simulate the life of a within the photon.
#[inline]
#[must_use]
pub fn emit_photon(scene: &Scene, light: &Light, mut rng: &mut ThreadRng) -> Sample {
    // Common constants.
    let bump_dist = scene.sett.bump_dist();
    let loop_limit = scene.sett.loop_limit();

    // Initialisation.
    let phot = light.emit(&mut rng, light.power() / scene.sett.num_phot() as f64);
    // let mat =

    Sample::new(0.0)
}
