//! Reaction simulation structure.

use crate::{access, clone, display_field_ln, kinetics::ConcBuilder};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable render settings structure.
#[load]
pub struct Settings {
    /// Minimum time-step.
    min_time_step: f64,
    /// Initial concentrations.
    concs: ConcBuilder,
}

impl Settings {
    clone!(min_time_step, f64);
    access!(concs, ConcBuilder);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "minimum time step", self.min_time_step, "s")?;
        // display_field!(fmt, "initial concentrations", self.concs)
        Ok(())
    }
}
