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
    /// List of diffusion coefficients, initial values, and sources/sinks.
    coeffs_values_sources: Redirect<Set<(PathBuf, Option<PathBuf>, Option<PathBuf>)>>,
    /// Reaction rate multiplier map.
    multipliers: PathBuf,
    /// Reactions.
    reactor: Redirect<ReactorLinker>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let grid = self.grid.load(in_dir)?;

        let coeffs_values_sources = self.coeffs_values_sources.load(in_dir)?;

        let mut list = Vec::with_capacity(coeffs_values_sources.len());
        for (name, (coeff_path, value_path, source_path)) in coeffs_values_sources {
            let coeffs = Array3::new_from_file(&in_dir.join(coeff_path))?;

            let values = if let Some(values) = value_path {
                Array3::new_from_file(&in_dir.join(values))?;
            } else {
                Array3::zeros(*coeffs.raw_dim())
            };

            let sources = if let Some(sources) = value_path {
                Array3::new_from_file(&in_dir.join(sources))?;
            } else {
                Array3::zeros(*coeffs.raw_dim())
            };

            list.push((name, (coeffs, values, sources)));
        }

        let multiplier = self.multiplier.load(in_dir)?;

        let reactor = self.reactor.load(in_dir)?;

        Ok(Self::Inst::new(
            sett,
            grid,
            Set::from_pairs(list)?,
            multiplier,
            reactor,
        ))
    }
}
