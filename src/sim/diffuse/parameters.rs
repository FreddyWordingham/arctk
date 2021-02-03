//! Runtime parameters.

use crate::{fmt_report, geom::Grid, sim::diffuse::Settings, util::fmt::Analyze};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Measurement grid.
    pub grid: Grid,
    /// Diffusion coefficents map.
    pub coeffs: Array3<f64>,
    /// Initial concentration map.
    pub init: Array3<f64>,
    /// Source map.
    pub sources: Array3<f64>,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        grid: Grid,
        coeffs: Array3<f64>,
        init: Array3<f64>,
        sources: Array3<f64>,
    ) -> Self {
        Self {
            sett,
            grid,
            coeffs,
            init,
            sources,
        }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid");
        fmt_report!(fmt, self.init.display(), "initial values");
        fmt_report!(fmt, self.coeffs.display(), "diffusion coefficients");
        Ok(())
    }
}
