//! Engine selection.

use crate::{
    ord::Build,
    sim::mcrt::{engines, Engine},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
#[file]
pub enum EngineBuilder {
    /// Standard sampling engine.
    Standard,
    /// Raman sampling engine.
    Raman,
    /// Fluorophore engine.
    Fluorophore,
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Standard => engines::standard,
            Self::Raman => engines::raman,
            Self::Fluorophore => engines::fluorophore,
        }
    }
}

impl Display for EngineBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Standard => write!(fmt, "Standard"),
            Self::Raman => write!(fmt, "Raman"),
            Self::Fluorophore => write!(fmt, "Raman"),
        }
    }
}
