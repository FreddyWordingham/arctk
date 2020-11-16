//! Parameters setup file.

use crate::{
    geom::{Grid, TreeSettings},
    opt::{AttributeSetup, Light, Material, SurfaceSetup},
    ord::Set,
    sim::mcrt::{Engine, SettingsSetup},
};

/// Named setup parameters.
/// Holds all simulation data, in human optimised form.
pub struct ParametersSetup {
    /// Materials.
    pub mats: Set<Material>,
    /// Attributes.
    pub attrs: Set<AttributeSetup>,
    /// Surfaces.
    pub surfs: Set<SurfaceSetup>,
    /// Illumination light.
    pub light: Light,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid.
    pub grid: Grid,
    /// Simulation specific settings.
    pub sett: SettingsSetup,
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
        attrs: Set<AttributeSetup>,
        surfs: Set<SurfaceSetup>,
        light: Light,
        tree: TreeSettings,
        grid: Grid,
        sett: SettingsSetup,
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
