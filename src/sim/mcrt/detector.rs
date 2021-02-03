//! Optical detectors.

use crate::data::Histogram;
use std::fmt::{Display, Error, Formatter};

/// Detectors.
pub enum Detector {
    /// Spectrometer.
    Spectrometer(Histogram),
}

impl Display for Detector {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Interface(in_mat, out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Spectrometer => {
                write!(fmt, "Spectrometer")
            }
        }
    }
}
