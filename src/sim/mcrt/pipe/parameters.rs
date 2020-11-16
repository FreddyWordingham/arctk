//! Startup parameters file.

use crate::{
    geom::{Grid, Tree, TreeSettings},
    opt::{Attribute, Light, Material, Surface},
    ord::Set,
    sim::mcrt::{Engine, Settings},
};

/// Functional input parameters.
/// Holds all simulation data, in ordered form.
pub struct Parameters<'a> {
    /// Materials.
    pub mats: Set<Material>,
    /// Attributes.
    pub attrs: Set<Attribute<'a>>,
    /// Surfaces.
    pub surfs: Set<Surface<'a>>,
    /// Main light.
    pub light: Light,
    /// Tree settings.
    pub tree: TreeSettings,
    /// Measurement grid.
    pub grid: Grid,
    /// Simulation specific settings.
    pub sett: Settings<'a>,
    /// Engine function.
    pub engine: Engine,
}

impl<'a> Parameters<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        mats: Set<Material>,
        attrs: Set<Attribute<'a>>,
        surfs: Set<Surface<'a>>,
        light: Light,
        tree: TreeSettings,
        grid: Grid,
        sett: Settings<'a>,
        engine: Engine,
    ) -> Self {
        debug_assert!(grid.num_cells() > 0);

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

    /// Grow the hit tree.
    #[inline]
    #[must_use]
    pub fn grow(&self) -> Tree {
        Tree::new(&self.tree, &self.surfs)
    }
}
