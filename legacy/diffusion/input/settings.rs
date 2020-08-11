//! Settings implementation.

use crate::{access, clone, display_field, display_field_ln, Aabb};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Diffusion simulation settings structure.
#[load]
pub struct Settings {
    /// Boundary.
    boundary: Aabb,
    /// Total time to simulate.
    total_time: f64,
    /// Fraction of the maximum timestep to take.
    step_frac: f64,
}

impl Settings {
    access!(boundary, Aabb);
    clone!(total_time, f64);
    clone!(step_frac, f64);
}

impl Display for Settings {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "boundary", &self.boundary, "m")?;
        display_field!(fmt, "total time", self.total_time, "s")?;
        display_field!(fmt, "step fraction", self.step_frac)
    }
}
