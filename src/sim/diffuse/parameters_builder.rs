//! Loadable parameters.

use crate::{
    fmt_report,
    geom::GridBuilder,
    ord::Build,
    sim::diffuse::{Parameters, Settings},
    util::datacube::display_datacube,
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Measurement grid settings.
    grid: GridBuilder,
    /// Initial concentration map.
    init: Array3<f64>,
    /// Diffusion coefficents map.
    coeffs: Array3<f64>,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: Settings,
        grid: GridBuilder,
        init: Array3<f64>,
        coeffs: Array3<f64>,
    ) -> Self {
        Self {
            sett,
            grid,
            init,
            coeffs,
        }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let grid = self.grid.build();
        let init = self.init;
        let coeffs = self.coeffs;

        Self::Inst::new(sett, grid, init, coeffs)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid settings");
        write!(fmt, "{:>32} : ", "initial values")?;
        display_datacube(fmt, &self.init)?;
        write!(fmt, "{:>32} : ", "diffusion coefficients")?;
        display_datacube(fmt, &self.coeffs)?;
        Ok(())
    }
}
