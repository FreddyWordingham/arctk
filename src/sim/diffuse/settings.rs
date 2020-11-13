//! Runtime settings structure.

use crate::clone;
use arctk_attr::load;

/// Loadable input diffusion settings structure.
#[load]
pub struct Settings {
    /// Time step multiplier (compared to max possible).
    step_multiplier: f64,
    /// Number of cells to calculate in one block.
    block_size: u64,
}

impl Settings {
    clone!(step_multiplier, f64);
    clone!(block_size, u64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(step_multiplier: f64, block_size: u64) -> Self {
        debug_assert!(step_multiplier > 0.0);
        debug_assert!(step_multiplier >= 1.0);
        debug_assert!(block_size > 0);

        Self {
            step_multiplier,
            block_size,
        }
    }
}
