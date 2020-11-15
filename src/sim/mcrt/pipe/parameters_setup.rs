//! Parameters setup file.

use crate::{
    geom::{Grid, Mesh, TreeSettings},
    opt::Light,
    ord::Set,
    sim::mcrt::{Catalogue, Engine, Parameters, Settings},
};

/// Named setup parameters.
/// Holds all simulation data, in human optimised form.
pub struct ParametersSetup {
    /// Engine function.
    engine: Engine,
    /// Tree settings.
    tree: TreeSettings,
    /// Surfaces.
    surfs: Set<Mesh>,
    /// Measurement grid.
    grid: Grid,
    /// Simulation specific settings.
    sett: Settings,
    /// Illumination light.
    light: Light,
}

impl ParametersSetup {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        engine: Engine,
        tree: TreeSettings,
        surfs: Set<Mesh>,
        grid: Grid,
        sett: Settings,
        light: Light,
    ) -> Self {
        Self {
            engine,
            tree,
            surfs,
            grid,
            sett,
            light,
        }
    }

    /// Setup the final parameters structure.
    #[inline]
    #[must_use]
    pub fn setup(self) -> (Parameters, Catalogue) {
        let engine = self.engine;
        let tree = self.tree;
        let (surfs, surfs_reg) = self.surfs.reg();
        let grid = self.grid;
        let sett = self.sett;
        let light = self.light;

        let cat = Catalogue::new(surfs_reg);
        let params = Parameters::new(engine, tree, grid, surfs, sett, light);

        (params, cat)
    }
}
