//! Loadable Engine selection.

use crate::{err::Error, fs::Load, sim::mcrt::EngineBuilder};
use arctk_attr::file;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Engine selection.
#[file]
pub enum EngineBuilderLoader {
    /// Standard sampling engine.
    Standard,
}

impl Load for EngineBuilderLoader {
    type Inst = EngineBuilder;

    #[inline]
    fn load(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Standard => Self::Inst::Standard,
        })
    }
}

impl Display for EngineBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Standard => write!(fmt, "Standard"),
        }
    }
}
