//! Engine selection.

use crate::{
    err::Error,
    file::Build,
    sim::mcrt::{engines, Engine},
};
use arctk_attr::load;
use std::path::Path;

/// Engine selection.
#[load]
#[derive(Clone)]
pub enum EngineBuilder {
    /// Standard sampling engine.
    Standard,
    /// Raman sampling engine.
    Raman,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Engine, Error> {
        match self {
            Self::Standard => Ok(engines::standard),
            Self::Raman => Ok(engines::raman),
        }
    }
}
