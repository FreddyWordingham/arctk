//! Photon-lifetime engine function.

use crate::mcrt::{Photon, Sample};

/// Simulate the life of a within the photon.
#[inline]
#[must_use]
pub fn the_photon_loop(_phot: Photon) -> Sample {
    Sample::new(0.0)
}
