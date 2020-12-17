//! Parameters linking file.

use crate::{
    geom::{Grid, SurfaceLinker, TreeSettings},
    ord::Set,
    sim::mcrt::{AttributeLinker, Engine, Light, Material, SettingsLinker},
};

/// Parameter linker structure.
/// Holds setup links to data loaded in memory.
pub struct ParametersLinker {
    /// Materials.
    pub mats: Set<Material>,
    /// Attributes.
    pub attrs: Set<AttributeLinker>,
    /// Surfaces.
    pub surfs: Set<SurfaceLinker>,
    /// Illumination light.
    pub light: Light,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid.
    pub grid: Grid,
    /// Simulation specific settings.
    pub sett: SettingsLinker,
    /// Engine function.
    pub engine: Engine,
}

impl ParametersLinker {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        mats: Set<Material>,
        attrs: Set<AttributeLinker>,
        surfs: Set<SurfaceLinker>,
        light: Light,
        tree: TreeSettings,
        grid: Grid,
        sett: SettingsLinker,
        engine: Engine,
    ) -> Self {
        Self {
            mats,
            attrs,
            surfs,
            light,
            tree,
            grid,
            sett,
            engine,
        }
    }
}
