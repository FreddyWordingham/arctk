//! Loadable parameters.

use crate::{
    fmt_report,
    geom::GridBuilder,
    ord::Build,
    sim::diffuse::{Parameters, Settings},
    util::fmt::Analyze,
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Measurement grid settings.
    grid: GridBuilder,
    /// Diffusion coefficents map.
    coeffs: Array3<f64>,
    /// Initial concentration map.
    init: Array3<f64>,
    /// Source/sink map.
    sources: Array3<f64>,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: Settings,
        grid: GridBuilder,
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

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let grid = self.grid.build();
        let coeffs = self.coeffs;
        let init = self.init;
        let sources = self.sources;

        Self::Inst::new(sett, grid, coeffs, init, sources)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid settings");
        fmt_report!(fmt, self.coeffs.display(), "diffusion coefficients");
        fmt_report!(fmt, self.init.display(), "initial values");
        fmt_report!(fmt, self.sources.display(), "sources/sinks");
        Ok(())
    }
}
