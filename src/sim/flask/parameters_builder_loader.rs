//! Loadable parameters.

use crate::{
    chem::ReactorLinker,
    err::Error,
    fs::{Load, Redirect},
    ord::ArrayLinker,
    sim::flask::{ParametersBuilder, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Initial concentrations.
    concs: Redirect<ArrayLinker>,
    /// Reactions.
    reactor: Redirect<ReactorLinker>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let concs = self.concs.load(in_dir)?;
        let reactor = self.reactor.load(in_dir)?;

        Ok(Self::Inst::new(sett, concs, reactor))
    }
}
