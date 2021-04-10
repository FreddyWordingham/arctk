//! Loadable Engine selection.

use crate::{
    err::Error,
    fs::Load,
    ord::Set,
    sim::mcrt::{EngineBuilder, FrameBuilder},
};
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
    /// Photography engine.
    Photo(Set<FrameBuilder>),
}

impl Load for EngineBuilderLoader {
    type Inst = EngineBuilder;

    #[inline]
    fn load(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Standard => Self::Inst::Standard,
            Self::Photo(frames) => Self::Inst::Photo(frames),
        })
    }
}

impl Display for EngineBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Standard => write!(fmt, "Standard"),
            Self::Photo(ref frames) => write!(fmt, "Photography ({})", frames.len()),
        }
    }
}
