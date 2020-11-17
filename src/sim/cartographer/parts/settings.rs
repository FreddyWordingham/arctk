//! Cartographer settings.

use crate::{access, clone, sim::cartographer::SuperSample};
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct Settings {
    /// Number of photons to simulate in each thread block.
    block_size: usize,
    /// Bump distance [m].
    bump_dist: f64,
    /// Super sampling.
    super_sampling: SuperSample,
}

impl Settings {
    clone!(block_size, usize);
    clone!(bump_dist, f64);
    access!(super_sampling, SuperSample);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(block_size: usize, bump_dist: f64, super_sampling: SuperSample) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);

        Self {
            block_size,
            bump_dist,
            super_sampling,
        }
    }
}
