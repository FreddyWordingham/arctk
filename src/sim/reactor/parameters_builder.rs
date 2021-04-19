//! Buildable parameters.

use crate::{
    chem::ReactorLinker,
    fmt_report,
    geom::GridBuilder,
    ord::{Build, Set},
    sim::reactor::{Parameters, Settings},
    util::fmt::DataCube,
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
    /// List of diffusion coefficients, initial values, and sources/sinks.
    coeffs_values_sources: Set<(Array3<f64>, Array3<f64>, Array3<f64>)>,
    /// Reaction rate multiplier map.
    multipliers: Array3<f64>,
    /// Reactions.
    reactor: ReactorLinker,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: Settings,
        grid: GridBuilder,
        coeffs_values_sources: Set<(Array3<f64>, Array3<f64>, Array3<f64>)>,
        multipliers: Array3<f64>,
        reactor: ReactorLinker,
    ) -> Self {
        Self {
            sett,
            grid,
            coeffs_values_sources,
            multipliers,
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
        let coeffs_values_sources = self.coeffs_values_sources;
        let multipliers = self.multipliers;
        let reactor = self.reactor;

        Self::Inst::new(sett, grid, coeffs_values_sources, multipliers, reactor)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid");

        for (name, &(ref coeffs, ref values, ref sources)) in self.coeffs_values_sources.map() {
            fmt_report!(
                fmt,
                DataCube::new(coeffs),
                &format!("{} diffusion coefficents", name)
            );
            fmt_report!(fmt, DataCube::new(values), &format!("init {} values", name));
            fmt_report!(
                fmt,
                DataCube::new(sources),
                &format!("source/sink {} values", name)
            );
        }

        fmt_report!(fmt, self.reactor, "reactor");
        Ok(())
    }
}
