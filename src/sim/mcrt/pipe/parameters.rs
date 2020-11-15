//! Startup parameters file.

use crate::{
    geom::{Grid, Mesh, Tree, TreeSettings},
    opt::Light,
    sim::mcrt::{Engine, Settings},
};

/// Functional input parameters.
/// Holds all simulation data, in ordered form.
pub struct Parameters {
    /// Engine function.
    pub engine: Engine,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid.
    pub grid: Grid,
    /// Surfaces.
    pub surfs: Vec<Mesh>,
    /// Simulation specific settings.
    pub sett: Settings,
    /// Main light.
    pub light: Light,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        engine: Engine,
        tree: TreeSettings,
        grid: Grid,
        surfs: Vec<Mesh>,
        sett: Settings,
        light: Light,
    ) -> Self {
        debug_assert!(!surfs.is_empty());
        debug_assert!(grid.num_cells() > 0);

        Self {
            engine,
            tree,
            grid,
            sett,
            surfs,
            light,
        }
    }

    /// Grow the hit tree.
    #[inline]
    #[must_use]
    pub fn grow(&self) -> Tree {
        Tree::new(&self.tree, &self.surfs)
    }
}
