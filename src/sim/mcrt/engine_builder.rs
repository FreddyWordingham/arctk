//! Engine selection.

use crate::{ord::Build, sim::mcrt::Engine};
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum EngineBuilder {
    /// Standard sampling engine.
    Standard,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Standard => Self::Inst::Standard,
        }
    }
}

impl Display for EngineBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Standard => write!(fmt, "Standard"),
        }
    }
}
