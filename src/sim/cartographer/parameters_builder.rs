//! Input parameter builder.

use crate::fmt_report;
use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::{GridBuilder, TreeSettings},
    sim::cartographer::{Parameters, Settings},
};
use arctk_attr::load;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Runtime parameters builder.
#[load]
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.build(in_dir)?;
        let tree = self.tree.build(in_dir)?;
        let grid = self.grid.build(in_dir)?.build(in_dir)?;

        Ok(Self::Inst::new(sett, tree, grid))
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.grid, "grid settings");
        Ok(())
    }
}
