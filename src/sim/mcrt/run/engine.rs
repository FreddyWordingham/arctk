//! Photon-lifetime engine function.

use crate::mcrt::{Photon, Sample, Scene};
use rand::rngs::ThreadRng;

/// Simulate the life of a within the photon.
#[inline]
#[must_use]
pub fn emit_photon(scene: &Scene, _rng: &mut ThreadRng, _phot: Photon) -> Sample {
    // Common constants.
    let _bump_dist = scene.sett.bump_dist();
    let _loop_limit = scene.sett.loop_limit();

    // Initialisation.

    Sample::new(0.0)
}
