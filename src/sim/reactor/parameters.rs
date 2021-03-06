//! Runtime parameters.

use crate::{
    chem::ReactorLinker, fmt_report, geom::Grid, ord::Set, sim::reactor::Settings,
    util::fmt::DataCube,
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Measurement grid.
    pub grid: Grid,
    /// List of diffusion coefficients, initial values, and sources/sinks.
    pub coeffs_values_sources: Set<(Array3<f64>, Array3<f64>, Array3<f64>)>,
    /// Reaction rate multiplier map.
    pub multipliers: Array3<f64>,
    /// Reactions.
    pub reactor: ReactorLinker,
}

impl Parameters {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        grid: Grid,
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

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
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
