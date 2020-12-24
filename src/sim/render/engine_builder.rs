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

impl Display for EngineBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Antler => write!(fmt, "Antler"),
        }
    }
}
