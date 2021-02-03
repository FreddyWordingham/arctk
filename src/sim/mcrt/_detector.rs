//! Optical detectors.

use crate::fmt_report;
use std::fmt::{Display, Formatter};

/// Detectors.
pub enum Detector {
    /// Spectrometer.
    Spectrometer(usize),
}

impl Display for Detector {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Spectrometer(id) => {
                fmt_report!(fmt, "Spectrometer", id);
                Ok(())
            }
        }
    }
}
