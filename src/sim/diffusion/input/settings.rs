//! Settings implementation.

use crate::{clone, display_field};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Diffusion simulation settings structure.
#[load]
pub struct Settings {
    /// Total time to simulate.
    total_time: f64,
}

impl Settings {
    clone!(total_time, f64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "total time", self.total_time, "s")
    }
}
