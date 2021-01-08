//! Engine selection.

use crate::{
    ord::Build,
    sim::render::{engines, Engine},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
#[file]
pub enum EngineBuilder {
    /// Antler rendering engine.
    Antler,
    /// Cross-section rendering engine.
    Cross,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Antler => engines::antler,
            Self::Cross => engines::cross,
        }
    }
}

impl Display for EngineBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Antler => write!(fmt, "Antler"),
            Self::Cross => write!(fmt, "Cross-Section"),
        }
    }
}
