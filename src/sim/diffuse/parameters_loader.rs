//! Loadable parameters.

use crate::{
    err::Error,
    fs::{Load, Redirect},
    sim::diffuse::{Parameters, Settings},
};
use arctk_attr::file;
use std::path::Path;

/// Loadable runtime parameters.
#[file]
pub struct ParametersLoader {
    /// Simulation specific settings.
    sett: Redirect<Settings>,
}

impl Load for ParametersLoader {
    type Inst = Parameters;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let sett = self.sett.load(in_dir)?;

        Ok(Self::Inst::new(sett))
    }
}
