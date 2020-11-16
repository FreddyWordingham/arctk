//! Parameters setup file.

use crate::{
    geom::{Grid, TreeSettings},
    opt::{AttributeLinker, Light, Material, SurfaceLinker},
    ord::Set,
    sim::mcrt::{Engine, SettingsLinker},
};

/// Named setup parameters.
/// Holds all simulation data, in human optimised form.
pub struct ParametersSetup {
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

impl ParametersSetup {
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
