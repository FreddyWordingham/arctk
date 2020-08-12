//! Settings structure.

use crate::{access, cartographer::Cast, clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of cells to map in each thread block.
    block_size: u64,
    /// Ray casting mode.
    cast: Cast,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(block_size, u64);
    access!(cast, Cast);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field!(fmt, "casting mode", &self.cast)
    }
}
