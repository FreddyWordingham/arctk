//! Optical detectors.

use crate::{data::Histogram, fmt_report};
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
            Self::Spectrometer(ref hist) => {
                fmt_report!(fmt, "Spectrometer", "kind");
                fmt_report!(fmt, hist, "histogram");
                Ok(())
            }
        }
    }
}
