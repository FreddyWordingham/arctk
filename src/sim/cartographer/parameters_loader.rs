//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{GridBuilder, TreeSettings},
    sim::cartographer::{ParametersBuilder, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Tree settings.
    tree: Redirect<TreeSettings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
}

impl Load for ParametersLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new())
    }
}
