//! MCRT settings.

use crate::clone;
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct Settings {
    /// Number of photons to simulate in each thread block.
    block_size: u64,
    /// Number of photons to simulate.
    num_phot: u64,
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(block_size, u64);
    clone!(num_phot, u64);
    clone!(bump_dist, f64);

    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(block_size: u64, num_phot: u64, bump_dist: f64) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(num_phot > 0);
        debug_assert!(bump_dist > 0.0);

        Self {
            block_size,
            num_phot,
            bump_dist,
        }
    }
}
