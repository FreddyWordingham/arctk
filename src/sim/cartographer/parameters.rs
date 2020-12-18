//! Input parameters.

use crate::fmt_report;
use crate::{
    file::Redirect,
    geom::{GridBuilder, TreeSettings},
    sim::cartographer::Settings,
};
use arctk_attr::load;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
#[load]
pub struct Parameters {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.grid, "grid settings");
        Ok(())
    }
}
