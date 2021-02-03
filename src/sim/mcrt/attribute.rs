//! Optical attributes.

use crate::{phys::Material, sim::mcrt::Detector};
use std::fmt::{Display, Error, Formatter};

/// Surface attributes.
pub enum Attribute<'a> {
    /// Material interface, inside material reference, outside material reference.
    Interface(&'a Material, &'a Material),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Detector.
    Detector(&'a Detector),
}

impl Display for Attribute<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Interface(in_mat, out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Detector(det) => {
                write!(fmt, "Spectrometer: {}", det)
            }
        }
    }
}
