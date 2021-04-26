//! Loadable Engine selection.

use crate::{
    err::Error,
    fs::{File, Load, Redirect},
    math::{FormulaBuilder, Pos3},
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
    Raman(Pos3),
    /// Photography engine.
    Photo(Redirect<Set<FrameBuilder>>),
    /// Fluorescence engine.
    Fluorescence(PathBuf, Redirect<FormulaBuilder>),
}

impl Load for EngineBuilderLoader {
    type Inst = EngineBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Standard => Self::Inst::Standard,
            Self::Raman(p) => Self::Inst::Raman(p),
            Self::Photo(frames) => Self::Inst::Photo(frames.load(in_dir)?),
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
            Self::Raman(ref _p) => write!(fmt, "Raman"),
            Self::Photo(ref _frames) => write!(fmt, "Photography"),
            Self::Fluorescence(..) => write!(fmt, "Fluorescence"),
        }
    }
}
