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
    /// Basic sampling engine.
    Basic,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Engine, Error> {
        match self {
            Self::Basic => Ok(engines::basic),
        }
    }
}
