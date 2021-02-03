//! Runtime parameters.

use crate::{
    fmt_report,
    geom::{Grid, SurfaceLinker, TreeSettings},
    ord::Set,
    phys::{LightLinker, Material},
    sim::mcrt::{AttributeLinkerLinker, Engine, Settings},
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
    pub attrs: Set<AttributeLinkerLinker>,
    /// Materials.
    pub mats: Set<Material>,
    /// Main light.
    pub light: LightLinker,
    /// Engine selection.
    pub engine: Engine,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub fn new(
        sett: Settings,
        tree: TreeSettings,
        grid: Grid,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinkerLinker>,
        mats: Set<Material>,
        light: LightLinker,
        engine: Engine,
    ) -> Self {
        Self {
            sett,
            tree,
            grid,
            surfs,
            attrs,
            mats,
            light,
            engine,
        }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.tree, "tree settings");
        fmt_report!(fmt, self.grid, "grid settings");
        fmt_report!(fmt, self.surfs, "surfaces");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.mats, "materials");
        fmt_report!(fmt, self.light, "light");
        fmt_report!(fmt, "{* POINTER LOADED *}", "engine");
        Ok(())
    }
}
