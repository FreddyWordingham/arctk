//! Rendering simulation structure.

use crate::{clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable render settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of pixels to simulate in each thread block.
    block_size: u64,
    /// Minimum photon weight.
    min_weight: f64,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(block_size, u64);
    clone!(min_weight, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field!(fmt, "minimum photon weight", self.min_weight)
    }
}
