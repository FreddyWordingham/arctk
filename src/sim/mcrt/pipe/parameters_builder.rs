//! Startup parameters file.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::{GridBuilder, MeshBuilder, TreeSettings},
    opt::{LightBuilder, MaterialBuilder},
    ord::Set,
    sim::mcrt::{AttributeSetup, EngineBuilder, ParametersSetup, Settings},
};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
/// Holds references to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Engine selection.
    engine: EngineBuilder,
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Surfaces.
    surfs: Redirect<Set<MeshBuilder>>,
    /// Materials.
    mats: Redirect<Set<Redirect<MaterialBuilder>>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeSetup>>,
    /// Main light.
    light: Redirect<LightBuilder>,
}

impl Build for ParametersBuilder {
    type Inst = ParametersSetup;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<ParametersSetup, Error> {
        let engine = self.engine.build(in_dir)?;
        let sett = self.sett.build(in_dir)?;
        let grid = self.grid.build(in_dir)?.build(in_dir)?;
        let tree = self.tree.build(in_dir)?;
        let surfs = self.surfs.build(in_dir)?.build(in_dir)?;
        let mats = self.mats.build(in_dir)?.build(in_dir)?.build(in_dir)?;
        let attrs = self.attrs.build(in_dir)?;
        let light = self.light.build(in_dir)?.build(in_dir)?;

        Ok(ParametersSetup::new(
            engine, sett, grid, tree, surfs, mats, attrs, light,
        ))
    }
}
