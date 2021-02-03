//! Optical detector builders.

use crate::{data::Histogram, fmt_report, ord::Build, sim::mcrt::Detector, tools::Range};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Detectors.
#[file]
pub enum DetectorBuilder {
    /// Spectrometer.
    Spectrometer([f64; 2], u64),
}

impl Build for DetectorBuilder {
    type Inst = Detector;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Spectrometer([min, max], bins) => {
                Self::Inst::Spectrometer(Histogram::new(min, max, bins))
            }
        }
    }
}

impl Display for DetectorBuilder {
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
