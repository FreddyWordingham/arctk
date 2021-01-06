//! Loadable parameters.

use crate::{
    fmt_report,
    geom::{SurfaceLinker, TreeSettings},
    ord::{Build, Link, Set},
    phys::{LightLinker, Material, MaterialBuilder},
    sim::mcrt::{AttributeLinker, Engine, EngineBuilder, Settings},
};
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    sett: Settings,
    /// Tree settings.
    tree: TreeSettings,
    /// Surfaces.
    surfs: Set<SurfaceLinker>,
    /// Attributes.
    attrs: Set<AttributeLinker>,
    /// Materials.
    mats: Set<Material>,
    /// Main light.
    light: LightLinker,
    /// Engine selection.
    engine: Engine,
}

impl Parameters {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        tree: TreeSettings,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinker>,
        mats: Set<Material>,
        light: LightLinker,
        engine: Engine,
    ) -> Self {
        Self {
            sett,
            tree,
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
        fmt_report!(fmt, self.surfs, "surfaces");
        fmt_report!(fmt, self.attrs, "attributes");
        fmt_report!(fmt, self.mats, "materials");
        fmt_report!(fmt, self.light, "light");
        fmt_report!(fmt, "{* POINTER LOADED *}", "engine");
        Ok(())
    }
}
