//! MCRT settings.

use crate::clone;
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct Settings {
    /// Number of photons to simulate.
    num_phot: u64,
    /// Number of photons to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
}

impl Settings {
    clone!(num_phot, u64);
    clone!(block_size, u64);
    clone!(bump_dist, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(num_phot: u64, block_size: u64, bump_dist: f64) -> Self {
        debug_assert!(num_phot > 0);
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);

        Self {
            num_phot,
            block_size,
            bump_dist,
        }
    }
}
