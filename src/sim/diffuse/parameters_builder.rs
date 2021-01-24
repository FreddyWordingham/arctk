//! Loadable parameters.

use crate::{
    fmt_report,
    geom::GridBuilder,
    ord::Build,
    sim::diffuse::{Parameters, Settings},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Measurement grid settings.
    grid: GridBuilder,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sett: Settings, grid: GridBuilder) -> Self {
        Self { sett, grid }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let grid = self.grid.build();

        Self::Inst::new(sett, grid)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.grid, "grid settings");
        Ok(())
    }
}
