//! Engine selection.

use crate::{
    math::FormulaBuilder,
    ord::{Build, Set},
    sim::mcrt::{Engine, FrameBuilder},
};
use ndarray::Array3;
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum EngineBuilder {
    /// Standard sampling engine.
    Standard,
    /// Raman engine.
    Raman,
    /// Photography engine.
    Photo(Set<FrameBuilder>),
    /// Fluorescence engine.
    Fluorescence(Array3<f64>, FormulaBuilder),
}

impl Build for EngineBuilder {
    type Inst = Engine;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Standard => Self::Inst::Standard,
            Self::Raman => Self::Inst::Raman,
            Self::Photo(frames) => Self::Inst::Photo(frames.build()),
            Self::Fluorescence(shift_map, conc_spec) => {
                Self::Inst::Fluorescence(shift_map, conc_spec.build())
            }
        }
    }
}

impl Display for EngineBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Standard => write!(fmt, "Standard"),
            Self::Raman => write!(fmt, "Raman"),
            Self::Photo(ref frames) => write!(fmt, "Photography ({})", frames.len()),
            Self::Fluorescence(..) => write!(fmt, "Fluorescence"),
        }
    }
}
