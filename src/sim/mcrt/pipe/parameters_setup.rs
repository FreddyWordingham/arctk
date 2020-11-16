//! Parameters setup file.

use crate::{
    geom::{Grid, TreeSettings},
    opt::{AttributeSetup, Light, Material, SurfaceSetup},
    ord::{Set, Setup},
    sim::mcrt::{Engine, Parameters, SettingsSetup},
};

/// Named setup parameters.
/// Holds all simulation data, in human optimised form.
pub struct ParametersSetup {
    /// Materials.
    mats: Set<Material>,
    /// Attributes.
    attrs: Set<AttributeSetup>,
    /// Surfaces.
    surfs: Set<SurfaceSetup>,
    /// Illumination light.
    light: Light,
    /// Tree settings.
    tree: TreeSettings,
    /// Measurement grid.
    grid: Grid,
    /// Simulation specific settings.
    sett: SettingsSetup,
    /// Engine function.
    engine: Engine,
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

    /// Setup the final parameters structure.
    #[inline]
    #[must_use]
    pub fn setup<'a>(self) -> Parameters<'a> {
        let mats = self.mats;

        let attrs = self
            .attrs
            .setup(&mats)
            .expect("Failed to link attribute set.");

        let surfs = self
            .surfs
            .setup(&attrs)
            .expect("Failed to link surface set.");

        let light = self.light;

        let tree = self.tree;

        let grid = self.grid;

        let sett = self.sett.setup(&mats).expect("Failed to link settings.");

        let engine = self.engine;

        Parameters::new(mats, attrs, surfs, light, tree, grid, sett, engine)
    }
}
