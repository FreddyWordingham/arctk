//! Optical attributes.

use crate::{data::Histogram, phys::Material};
use std::fmt::{Display, Error, Formatter};

/// Surface attributes.
pub enum Attribute<'a> {
    /// Material interface, inside material reference, outside material reference.
    Interface(&'a Material, &'a Material),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer.
    Spectrometer(Histogram),
}

impl Display for Attribute<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Interface(ref in_mat, ref out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Spectrometer(hist) => {
                write!(
                    fmt,
                    "Spectrometer: [{}, {}] ({})",
                    hist.binner().range().min(),
                    hist.binner().range().max(),
                    hist.binner().bins()
                )
            }
        }
    }
}
