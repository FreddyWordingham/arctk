//! Input parameters.

// use crate::fmt_report;
use crate::{
    geom::{Grid, SurfaceLinker, TreeSettings},
    ord::Set,
    phys::AttributeLinker,
    sim::cartographer::Settings,
};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid settings.
    pub grid: Grid,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Attributes.
    pub attrs: Set<AttributeLinker>,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sett: Settings,
        tree: TreeSettings,
        grid: Grid,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinker>,
    ) -> Self {
        Self {
            sett,
            tree,
            grid,
            surfs,
            attrs,
        }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        // fmt_report!(fmt, self.sett, "settings");
        // fmt_report!(fmt, self.tree, "tree settings");
        // fmt_report!(fmt, self.grid, "grid settings");
        Ok(())
    }
}
