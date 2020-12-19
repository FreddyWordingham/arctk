//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{GridBuilder, SurfaceLinkerLoader, TreeSettings},
    ord::Set,
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
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new())
    }
}
