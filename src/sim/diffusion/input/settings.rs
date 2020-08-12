//! Settings implementation.

use crate::{access, clone, display_field, display_field_ln, Aabb};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Diffusion simulation settings structure.
#[load]
pub struct Settings {
    /// Boundary.
    boundary: Aabb,
    /// Number of dumps to make during the simulation.
    num_dumps: u64,
    /// Total time to simulate.
    total_time: f64,
    /// Fraction of the maximum timestep to take.
    step_frac: f64,
}

impl Settings {
    access!(boundary, Aabb);
    clone!(num_dumps, u64);
    clone!(total_time, f64);
    clone!(step_frac, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "boundary", &self.boundary)?;
        display_field_ln!(fmt, "number of dumps", self.num_dumps)?;
        display_field_ln!(fmt, "total time", self.total_time, "s")?;
        display_field!(fmt, "step fraction", self.step_frac)
    }
}
