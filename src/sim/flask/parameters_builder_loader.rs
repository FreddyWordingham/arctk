//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    geom::{GridBuilder, SurfaceLinkerLoader, TreeSettings},
    ord::Set,
    phys::{LightLinkerBuilderLoader, MaterialBuilder},
    sim::mcrt::{AttributeLinker, EngineBuilder, ParametersBuilder, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
    /// Initial concentrations.
    concs: Redirect<Concentrations>,
    /// Reactions.
    reactor: Redirect<ReactorLinker>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;
        let concs = self.tree.load(in_dir)?;
        let reactor = self.grid.load(in_dir)?;

        Ok(Self::Inst::new(sett, concs, reactor))
    }
}
