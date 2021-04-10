//! Engine function handler.

use crate::{
    phys::Photon,
    sim::mcrt::{engines, Input, Output},
};
use rand::rngs::ThreadRng;
use std::fmt::{Display, Error, Formatter};

/// Engine selection.
pub enum Engine {
    /// Standard sampling engine.
    Standard,
}

impl Engine {
    /// Run the engine for a single photon.
    #[inline]
    pub fn run(&self, input: &Input, data: &mut Output, rng: &mut ThreadRng, phot: Photon) {
        match *self {
            Self::Standard => engines::standard(input, data, rng, phot),
        }
    }
}

impl Display for Engine {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Standard => write!(fmt, "Standard"),
        }
    }
}

// /// MCRT sampling engine function type.
// pub type Engine<T> = fn(info: &T, input: &Input, &mut ThreadRng, phot: Photon, data: &mut Output);
