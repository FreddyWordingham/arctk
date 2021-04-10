//! Engine selection.

use crate::{
    ord::Build,
    sim::mcrt::{Engine, FrameBuilder},
};
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum EngineBuilder {
    /// Standard sampling engine.
    Standard,
    /// Photography engine.
    Photo(Vec<FrameBuilder>),
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Standard => Self::Inst::Standard,
            Self::Photo(frames) => {
                let mut fs = Vec::with_capacity(frames.len());
                for frame in frames {
                    fs.push(frame.build());
                }
                Self::Inst::Photo(fs)
            }
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
