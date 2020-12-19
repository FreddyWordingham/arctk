//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{GridBuilder, SurfaceLinkerLoader, TreeSettings},
    ord::Set,
    phys::AttributeLinker,
    sim::cartographer::{ParametersBuilder, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Surfaces.
    surfs: Redirect<Set<SurfaceLinkerLoader>>,
    /// Attributes.
    attrs: Redirect<Set<AttributeLinker>>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let tree = self.tree.load(in_dir)?;
        let grid = self.grid.load(in_dir)?;
        let surfs = self.surfs.load(in_dir)?.load(in_dir)?;
        let attrs = self.attrs.load(in_dir)?;

        Ok(Self::Inst::new(sett, tree, grid, surfs, attrs))
    }
}
