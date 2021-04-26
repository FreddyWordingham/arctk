//! Loadable Engine selection.

use crate::{
    err::Error,
    fs::{File, Load, Redirect},
    math::FormulaBuilder,
    ord::Set,
    sim::mcrt::{EngineBuilder, FrameBuilder},
};
use arctk_attr::file;
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

/// Engine selection.
#[file]
pub enum EngineBuilderLoader {
    /// Standard sampling engine.
    Standard,
    /// Raman engine.
    Raman,
    /// Photography engine.
    Photo(Set<FrameBuilder>),
    /// Fluorescence engine.
    Fluorescence(PathBuf, Redirect<FormulaBuilder>),
}

impl Load for EngineBuilderLoader {
    type Inst = EngineBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Standard => Self::Inst::Standard,
            Self::Raman => Self::Inst::Raman,
            Self::Photo(frames) => Self::Inst::Photo(frames),
            Self::Fluorescence(shift_map, conc_spec) => Self::Inst::Fluorescence(
                Array3::new_from_file(&in_dir.join(shift_map))?,
                conc_spec.load(in_dir)?,
            ),
        })
    }
}

impl Display for EngineBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Standard => write!(fmt, "Standard"),
            Self::Raman => write!(fmt, "Raman"),
            Self::Photo(ref frames) => write!(fmt, "Photography ({})", frames.len()),
            Self::Fluorescence(..) => write!(fmt, "Fluorescence"),
        }
    }
}
