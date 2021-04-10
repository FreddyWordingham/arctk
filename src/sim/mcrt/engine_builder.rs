//! Engine selection.

use crate::{
    ord::{Build, Set},
    sim::mcrt::{Engine, FrameBuilder},
};
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum EngineBuilder {
    /// Standard sampling engine.
    Standard,
    /// Photography engine.
    Photo(Set<FrameBuilder>),
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Standard => Self::Inst::Standard,
            Self::Photo(frames) => Self::Inst::Photo(frames.build()),
        }
    }
}

impl Display for EngineBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Standard => write!(fmt, "Standard"),
            Self::Photo(ref frames) => write!(fmt, "Photography ({})", frames.len()),
        }
    }
}
