//! Loadable parameters.

use crate::{
    chem::ReactorLinker,
    err::Error,
    fs::{File, Load, Redirect},
    ord::Set,
    sim::reactor::{Parameters, Settings},
};
use arctk_attr::file;
use ndarray::Array3;
use std::path::{Path, PathBuf};

/// Loadable runtime parameters.
#[file]
pub struct ParametersLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Initial concentration maps.
    init: Redirect<Set<PathBuf>>,
    /// Reactions.
    reactor: Redirect<ReactorLinker>,
}

impl Load for ParametersLoader {
    type Inst = Parameters;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;

        let init = self.init.load(in_dir)?;
        let mut list = Vec::with_capacity(init.len());
        for (name, path) in init {
            list.push((name, Array3::new_from_file(&in_dir.join(path))?));
        }

        let reactor = self.reactor.load(in_dir)?;

        Ok(Self::Inst::new(sett, Set::from_pairs(list)?, reactor))
    }
}
