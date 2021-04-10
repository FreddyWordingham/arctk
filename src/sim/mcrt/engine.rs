//! Engine function handler.

use crate::{
    ord::Set,
    phys::Photon,
    sim::mcrt::{engines, Frame, Input, Output},
};
use rand::rngs::ThreadRng;
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum Engine {
    /// Standard sampling engine.
    Standard,
    /// Photography engine.
    Photo(Set<Frame>),
}

impl Engine {
    /// Run the engine for a single photon.
    #[inline]
    pub fn run(&self, input: &Input, data: &mut Output, rng: &mut ThreadRng, phot: Photon) {
        match self {
            Self::Standard => engines::standard(input, data, rng, phot),
            Self::Photo(ref frames) => engines::photo(frames, input, data, rng, phot),
        }
    }
}

impl Display for Engine {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            Self::Standard => write!(fmt, "Standard"),
            Self::Photo(ref frames) => write!(fmt, "Photography ({})", frames.len()),
        }
    }
}
