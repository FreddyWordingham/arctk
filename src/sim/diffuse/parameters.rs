//! Runtime parameters.

use crate::{fmt_report, geom::Grid, sim::diffuse::Settings};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Measurement grid.
    pub grid: Grid,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(sett: Settings, grid: Grid) -> Self {
        Self { sett, grid }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid");
        Ok(())
    }
}
