//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::GridBuilder,
    sim::diffuse::{ParametersBuilder, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let grid = self.grid.load(in_dir)?;

        Ok(Self::Inst::new(sett, grid))
    }
}
