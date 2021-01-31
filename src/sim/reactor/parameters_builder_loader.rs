//! Loadable parameters.

use crate::{
    chem::ReactorLinker,
    err::Error,
    fs::{File, Load, Redirect},
    geom::GridBuilder,
    ord::Set,
    sim::reactor::{ParametersBuilder, Settings},
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
    /// Initial concentrations and diffusion coefficient maps.
    values_coeffs: Redirect<Set<(PathBuf, PathBuf)>>,
    /// Reactions.
    reactor: Redirect<ReactorLinker>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let grid = self.grid.load(in_dir)?;

        let values_coeffs = self.values_coeffs.load(in_dir)?;
        let mut list = Vec::with_capacity(values_coeffs.len());
        for (name, (value_path, coeff_path)) in values_coeffs {
            list.push((
                name,
                (
                    Array3::new_from_file(&in_dir.join(value_path))?,
                    Array3::new_from_file(&in_dir.join(coeff_path))?,
                ),
            ));
        }

        let reactor = self.reactor.load(in_dir)?;

        Ok(Self::Inst::new(sett, grid, Set::from_pairs(list)?, reactor))
    }
}
