//! Settings structure.

use crate::{clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of photons to simulate in each thread block.
    block_size: u64,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(block_size, u64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field!(fmt, "block size", self.block_size)
    }
}
