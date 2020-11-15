//! Startup parameters file.

use crate::{
    geom::{Grid, Mesh, Tree, TreeSettings},
    opt::{Light, Material},
    sim::mcrt::{Attribute, Engine, Settings},
};

/// Functional input parameters.
/// Holds all simulation data, in ordered form.
pub struct Parameters {
    /// Engine function.
    pub engine: Engine,
    /// Simulation specific settings.
    pub sett: Settings,
    /// Measurement grid.
    pub grid: Grid,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Surfaces.
    pub surfs: Vec<Mesh>,
    /// Materials.
    pub mats: Vec<Material>,
    /// Attributes.
    pub attrs: Vec<Attribute>,
    /// Main light.
    pub light: Light,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        engine: Engine,
        sett: Settings,
        grid: Grid,
        tree: TreeSettings,
        surfs: Vec<Mesh>,
        mats: Vec<Material>,
        attrs: Vec<Attribute>,
        light: Light,
    ) -> Self {
        debug_assert!(grid.num_cells() > 0);
        debug_assert!(!surfs.is_empty());
        debug_assert!(!attrs.is_empty());
        debug_assert!(!mats.is_empty());

        Self {
            engine,
            tree,
            grid,
            sett,
            surfs,
            mats,
            attrs,
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
