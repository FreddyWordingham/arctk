//! Engine function handler.

use crate::{
    math::Formula,
    ord::Set,
    phys::Photon,
    sim::mcrt::{engines, Frame, Input, Output},
};
use ndarray::Array3;
use rand::rngs::ThreadRng;
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum Engine {
    /// Standard sampling engine.
    Standard,
    /// Ramanisation engine.
    Raman,
    /// Photography engine.
    Photo(Set<Frame>),
    /// Fluorescence engine.
    Fluorescence(Array3<f64>, Formula),
}

impl Engine {
    /// Run the engine for a single photon.
    #[inline]
    pub fn run(&self, input: &Input, data: &mut Output, rng: &mut ThreadRng, phot: Photon) {
        match *self {
            Self::Standard => engines::standard(input, data, rng, phot),
            Self::Raman => engines::raman(input, data, rng, phot),
            Self::Photo(ref frames) => engines::photo(frames, input, data, rng, phot),
            Self::Fluorescence(ref shift_map, ref conc_spec) => {
                engines::fluorescence(shift_map, conc_spec, input, data, rng, phot)
            }
        }
    }
}

impl Display for Engine {
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
