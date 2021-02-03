//! Simulation input.

use crate::{fmt_report, geom::Grid, sim::diffuse::Settings, util::fmt::Analyze};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Diffuse simulation resources conglomerate.
pub struct Input<'a> {
    /// Map of diffusion coeffs.
    pub coeffs: &'a Array3<f64>,
    /// Map sources/sinks.
    pub sources: &'a Array3<f64>,
    /// Measurement grid.
    pub grid: &'a Grid,
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        coeffs: &'a Array3<f64>,
        sources: &'a Array3<f64>,
        grid: &'a Grid,
        sett: &'a Settings,
    ) -> Self {
        Self {
            coeffs,
            sources,
            grid,
            sett,
        }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.coeffs.display(), "diffusion coefficients");
        fmt_report!(fmt, self.sources.display(), "sources/sinks");
        fmt_report!(fmt, self.grid, "measurement grid");
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
