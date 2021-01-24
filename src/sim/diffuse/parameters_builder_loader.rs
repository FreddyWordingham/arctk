//! Loadable parameters.

use crate::{
    err::Error,
    fs::{File, Load, Redirect},
    geom::GridBuilder,
    sim::diffuse::{ParametersBuilder, Settings},
};
use arctk_attr::file;
use ndarray::Array3;
use std::path::{Path, PathBuf};

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Measurement grid settings.
    grid: Redirect<GridBuilder>,
    /// Initial concentration map.
    init: PathBuf,
    /// Diffusion coefficents map.
    coeffs: PathBuf,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let grid = self.grid.load(in_dir)?;
        let init = Array3::new_from_file(&in_dir.join(self.init))?;
        let coeffs = Array3::new_from_file(&in_dir.join(self.coeffs))?;

        Ok(Self::Inst::new(sett, grid, init, coeffs))
    }
}
