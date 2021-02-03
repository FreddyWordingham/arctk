//! Optical detector setup information.

use crate::{fmt_report, tools::Range};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Detectors.
#[file]
pub enum DetectorSetup {
    /// Spectrometer.
    Spectrometer([f64; 2], u64),
}

impl Display for DetectorSetup {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Spectrometer([min, max], bins) => {
                fmt_report!(fmt, "Spectrometer", "kind");
                fmt_report!(fmt, Range::new(min, max), "range");
                fmt_report!(fmt, bins, "bins");
                Ok(())
            }
        }
    }
}
