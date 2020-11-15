//! Startup parameters file.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::{GridBuilder, MeshBuilder, TreeSettings},
    opt::LightBuilder,
    ord::Set,
    sim::mcrt::{EngineBuilder, ParametersSetup, Settings},
};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
/// Holds references to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Engine selection.
    engine: EngineBuilder,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Surfaces.
    surfs: Redirect<Set<MeshBuilder>>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Main light.
    light: Redirect<LightBuilder>,
}

impl Build for ParametersBuilder {
    type Inst = ParametersSetup;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<ParametersSetup, Error> {
        let engine = self.engine.build(in_dir)?;
        let tree = self.tree.build(in_dir)?;
        let surfs = self.surfs.build(in_dir)?.build(in_dir)?;
        let grid = self.grid.build(in_dir)?.build(in_dir)?;
        let sett = self.sett.build(in_dir)?;
        let light = self.light.build(in_dir)?.build(in_dir)?;

        Ok(ParametersSetup::new(engine, tree, surfs, grid, sett, light))
    }
}
