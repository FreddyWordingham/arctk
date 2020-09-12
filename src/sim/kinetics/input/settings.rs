//! Reaction simulation structure.

use crate::{clone, display_field, display_field_ln};
use arctk_attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable render settings structure.
#[load]
pub struct Settings {
    /// Time-step.
    dt: f64,
    /// Minimum time-step size.
    min_step: f64,
}

impl Settings {
    clone!(dt, f64);
    clone!(min_step, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "time-step", self.dt, "s")?;
        display_field!(fmt, "minimum time-step", self.min_step, "s")
    }
}
