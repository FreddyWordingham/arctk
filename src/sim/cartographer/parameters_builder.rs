//! Buildable parameters.

use crate::{
    fmt_report,
    geom::{GridBuilder, SurfaceLinker, TreeSettings},
    ord::{Build, Set},
    sim::cartographer::{AttributeLinker, Parameters, Settings},
};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters builder.
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Tree settings.
    tree: TreeSettings,
    /// Measurement grid settings.
    grid: GridBuilder,
    /// Surfaces.
    surfs: Set<SurfaceLinker>,
    /// Attributes.
    attrs: Set<AttributeLinker>,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        tree: TreeSettings,
        grid: GridBuilder,
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

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let tree = self.tree;
        let grid = self.grid.build();
        let surfs = self.surfs;
        let attrs = self.attrs;

        Self::Inst::new(sett, tree, grid, surfs, attrs)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.grid, "grid settings");
        fmt_report!(fmt, self.surfs, "surfaces");
        fmt_report!(fmt, self.attrs, "attributes");
        Ok(())
    }
}
