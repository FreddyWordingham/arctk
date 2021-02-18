//! Loadable parameters.

use crate::{
    chem::ReactorLinker,
    err::Error,
    fs::{Load, Redirect},
    ord::ArrayLinker,
    sim::flask::{Parameters, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Initial values.
    init: ArrayLinker,
    /// Sources.
    sources: ArrayLinker,
    /// Reactions.
    reactor: Redirect<ReactorLinker>,
}

impl Load for ParametersLoader {
    type Inst = Parameters;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let init = self.init;
        let sources = self.sources;
        let reactor = self.reactor.load(in_dir)?;

        Ok(Self::Inst::new(sett, init, sources, reactor))
    }
}
