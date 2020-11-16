//! Startup parameters file.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::{GridBuilder, TreeSettings},
    opt::{AttributeLinker, LightBuilder, MaterialBuilder, SurfaceBuilder},
    ord::Set,
    sim::mcrt::{EngineBuilder, ParametersSetup, SettingsLinker},
};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
/// Holds references to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Materials.
    mats: Redirect<Set<Redirect<MaterialBuilder>>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeLinker>>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceBuilder>>,
    /// Main light.
    light: Redirect<LightBuilder>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Simulation specific settings.
    sett: Redirect<SettingsLinker>,
    /// Engine selection.
    engine: EngineBuilder,
}

impl Build for ParametersBuilder {
    type Inst = ParametersSetup;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<ParametersSetup, Error> {
        let mats = self.mats.build(in_dir)?.build(in_dir)?.build(in_dir)?;
        let attrs = self.attrs.build(in_dir)?;
        let surfs = self.surfs.build(in_dir)?.build(in_dir)?;
        let light = self.light.build(in_dir)?.build(in_dir)?;
        let tree = self.tree.build(in_dir)?;
        let grid = self.grid.build(in_dir)?.build(in_dir)?;
        let sett = self.sett.build(in_dir)?;
        let engine = self.engine.build(in_dir)?;

        Ok(ParametersSetup::new(
            mats, attrs, surfs, light, tree, grid, sett, engine,
        ))
    }
}
