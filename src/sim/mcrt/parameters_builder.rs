//! Buildable parameters.

use crate::{
    fmt_report,
    geom::{GridBuilder, SurfaceLinker, TreeSettings},
    ord::{Build, Set},
    phys::{LightLinkerBuilder, MaterialBuilder},
    sim::mcrt::{AttributeLinker, DetectorBuilder, EngineBuilder, Parameters, Settings},
};
use std::fmt::{Display, Error, Formatter};

/// Buildable runtime parameters.
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
    /// Materials.
    mats: Set<MaterialBuilder>,
    /// Detectors.
    detectors: Set<DetectorBuilder>,
    /// Main light.
    light: LightLinkerBuilder,
    /// Engine selection.
    engine: EngineBuilder,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        tree: TreeSettings,
        grid: GridBuilder,
        surfs: Set<SurfaceLinker>,
        attrs: Set<AttributeLinker>,
        mats: Set<MaterialBuilder>,
        detectors: Set<DetectorBuilder>,
        light: LightLinkerBuilder,
        engine: EngineBuilder,
    ) -> Self {
        Self {
            sett,
            tree,
            grid,
            surfs,
            attrs,
            mats,
            detectors,
            light,
            engine,
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
        let mats = self.mats.build();
        let detectors = self.detectors.build();
        let light = self.light.build();
        let engine = self.engine.build();

        Self::Inst::new(
            sett, tree, grid, surfs, attrs, mats, detectors, light, engine,
        )
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
        fmt_report!(fmt, self.mats, "materials");
        fmt_report!(fmt, self.detectors, "detectors");
        fmt_report!(fmt, self.light, "light");
        fmt_report!(fmt, self.engine, "engine");
        Ok(())
    }
}
