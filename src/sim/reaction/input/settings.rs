//! Reaction simulation structure.

use crate::{clone, display_field};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable render settings structure.
#[load]
pub struct Settings {
    /// Minimum time-step.
    min_time_step: f64,
}

impl Settings {
    clone!(min_time_step, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "minimum time step", self.min_time_step, "s")
    }
}
