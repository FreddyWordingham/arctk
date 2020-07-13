//! Settings implementation.

use crate::{clone, display_field, display_field_ln, render::Order};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering settings structure.
#[load]
pub struct Settings {
    /// Bump distance [m].
    bump_dist: f64,
    /// Number of pixels to simulate in each thread block.
    block_size: u64,
    /// Minimum photon weight.
    min_weight: f64,
    /// Optional live update size.
    live: Option<u64>,
    /// Rendering order.
    order: Order,
}

impl Settings {
    clone!(bump_dist, f64);
    clone!(block_size, u64);
    clone!(min_weight, f64);
    clone!(live, Option<u64>);
    clone!(order, Order);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "bump distance", self.bump_dist, "m")?;
        display_field_ln!(fmt, "block size", self.block_size)?;
        display_field_ln!(fmt, "minimum photon weight", self.min_weight)?;
        if let Some(update_size) = self.live {
            display_field_ln!(fmt, "live", update_size)?;
        } else {
            display_field_ln!(fmt, "live", "[OFF]")?;
        }
        display_field!(fmt, "rendering order", &self.order)
    }
}
