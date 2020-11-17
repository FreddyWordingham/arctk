//! Startup parameters file.

use crate::{
    err::Error,
    file::{Build, Redirect},
    geom::{GridBuilder, SurfaceBuilder, TreeSettings},
    ord::Set,
    sim::cartographer::{Attribute, EngineBuilder, ParametersLinker, Settings},
};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
/// Holds paths to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Attributes.
    attrs: Redirect<Set<Attribute>>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceBuilder>>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Engine selection.
    engine: EngineBuilder,
}

impl Build for ParametersBuilder {
    type Inst = ParametersLinker;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let attrs = self.attrs.build(in_dir)?;
        let surfs = self.surfs.build(in_dir)?.build(in_dir)?;
        let tree = self.tree.build(in_dir)?;
        let grid = self.grid.build(in_dir)?.build(in_dir)?;
        let sett = self.sett.build(in_dir)?;
        let engine = self.engine.build(in_dir)?;

        Ok(Self::Inst::new(attrs, surfs, tree, grid, sett, engine))
    }
}
