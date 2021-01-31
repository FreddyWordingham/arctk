//! Buildable parameters.

use crate::{
    chem::ReactorLinker,
    fmt_report,
    geom::GridBuilder,
    ord::{Build, Set},
    sim::reactor::{Parameters, Settings},
    util::fmt::datacube::display_datacube,
};
use arctk_attr::file;
use ndarray::Array3;
use std::fmt::{Display, Formatter};

/// Buildable runtime parameters.
#[file]
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Measurement grid settings.
    grid: GridBuilder,
    /// Initial concentrations and diffusion coefficient maps.
    values_coeffs: Set<(Array3<f64>, Array3<f64>)>,
    /// Reactions.
    reactor: ReactorLinker,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        sett: Settings,
        grid: GridBuilder,
        values_coeffs: Set<(Array3<f64>, Array3<f64>)>,
        reactor: ReactorLinker,
    ) -> Self {
        Self {
            sett,
            grid,
            values_coeffs,
            reactor,
        }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let grid = self.grid.build();
        let values_coeffs = self.values_coeffs;
        let reactor = self.reactor;

        Self::Inst::new(sett, grid, values_coeffs, reactor)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid");

        for (name, (values, coeffs)) in self.values_coeffs.map() {
            write!(fmt, "{:>32} : ", &format!("init {} values", name))?;
            display_datacube(fmt, values)?;
            write!(fmt, "{:>32} : ", &format!("{} diffusion coefficents", name))?;
            display_datacube(fmt, coeffs)?;
        }

        fmt_report!(fmt, self.reactor, "reactor");
        Ok(())
    }
}
