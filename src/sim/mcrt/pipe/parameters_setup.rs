//! Parameters setup file.

use crate::{
    geom::{Grid, Mesh, TreeSettings},
    opt::{Light, Material},
    ord::{Set, Setup},
    sim::mcrt::{AttributeSetup, Catalogue, Engine, Parameters, Settings},
};

/// Named setup parameters.
/// Holds all simulation data, in human optimised form.
pub struct ParametersSetup {
    /// Engine function.
    engine: Engine,
    /// Simulation specific settings.
    sett: Settings,
    /// Measurement grid.
    grid: Grid,
    /// Tree settings.
    tree: TreeSettings,
    /// Surfaces.
    surfs: Set<Mesh>,
    /// Materials.
    mats: Set<Material>,
    /// Attributes.
    attrs: Set<AttributeSetup>,
    /// Illumination light.
    light: Light,
}

impl ParametersSetup {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        engine: Engine,
        sett: Settings,
        grid: Grid,
        tree: TreeSettings,
        surfs: Set<Mesh>,
        mats: Set<Material>,
        attrs: Set<AttributeSetup>,
        light: Light,
    ) -> Self {
        Self {
            engine,
            sett,
            grid,
            tree,
            surfs,
            mats,
            attrs,
            light,
        }
    }

    /// Setup the final parameters structure.
    #[inline]
    #[must_use]
    pub fn setup(self) -> (Parameters, Catalogue) {
        let engine = self.engine;
        let sett = self.sett;
        let grid = self.grid;
        let tree = self.tree;
        let (surfs, surf_reg) = self.surfs.reg();
        let (mats, mat_reg) = self.mats.reg();
        let (attrs, attr_reg) = self.attrs.setup(&mat_reg).reg();
        let light = self.light;

        let cat = Catalogue::new(surf_reg, mat_reg, attr_reg);
        let params = Parameters::new(engine, sett, grid, tree, surfs, mats, attrs, light);

        (params, cat)
    }
}
