//! Settings structure.

use crate::{clone, display_field};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable technical settings structure.
#[load]
pub struct Settings {
    /// Number of cells to map in each thread block.
    block_size: u64,
}

impl Settings {
    clone!(block_size, u64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "block size", self.block_size)
    }
}
