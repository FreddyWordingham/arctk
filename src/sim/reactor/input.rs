//! Simulation input.

use crate::{chem::Reactor, fmt_report, geom::Grid, ord::Register, sim::reactor::Settings,util::datacube::display_datatesseract};
use ndarray::Array4;
use std::fmt::{Display, Error, Formatter};

/// Reactor simulation resources conglomerate.
pub struct Input<'a> {
    /// Register of known species.
    pub specs: &'a Register,
    /// Reactor processor.
    pub reactor: &'a Reactor,
    /// Map of diffusion coeffs.
    pub coeffs: &'a Array4<f64>,
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
        specs: &'a Register,
        reactor: &'a Reactor,
        coeffs: &'a Array4<f64>,
        grid: &'a Grid,
        sett: &'a Settings,
    ) -> Self {
        Self {
            specs,
            reactor,
            coeffs,
            grid,
            sett,
        }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.specs, "species");
        fmt_report!(fmt, self.reactor, "reactor");
        write!(fmt, "{:>32} : ", "diffusion coefficients")?;
        display_datatesseract(fmt, self.coeffs)?;
        fmt_report!(fmt, self.grid, "measurement grid");
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
