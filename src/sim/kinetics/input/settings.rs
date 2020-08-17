//! Reaction simulation structure.

use crate::{clone, display_field};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable render settings structure.
#[load]
pub struct Settings {
    /// Time-step.
    dt: f64,
}

impl Settings {
    clone!(dt, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(fmt, "time-step", self.dt, "s")
    }
}
