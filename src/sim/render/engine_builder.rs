//! Engine selection.

use crate::{
    ord::Build,
    sim::render::{engines, Engine},
};
use arctk_attr::file;

/// Engine selection.
#[file]
pub enum EngineBuilder {
    /// Antler rendering engine.
    Antler,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Antler => engines::antler,
        }
    }
}
