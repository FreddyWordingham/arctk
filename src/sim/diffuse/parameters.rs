//! Runtime parameters.

use crate::{fmt_report, geom::Grid, sim::diffuse::Settings, util::datacube::display_datacube};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Measurement grid.
    pub grid: Grid,
    /// Initial concentration map.
    init: Array3<f64>,
    /// Diffusion coefficents map.
    coeffs: Array3<f64>,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(sett: Settings, grid: Grid, init: Array3<f64>, coeffs: Array3<f64>) -> Self {
        Self {
            sett,
            grid,
            init,
            coeffs,
        }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid");
        writeln!(fmt, "initial values...")?;
        display_datacube(fmt, &self.init)?;
        writeln!(fmt, "diffusion coefficients...")?;
        display_datacube(fmt, &self.coeffs)?;
        Ok(())
    }
}
