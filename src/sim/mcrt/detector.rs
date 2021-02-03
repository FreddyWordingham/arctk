//! Optical detectors.

use crate::{data::Histogram, err::Error, fmt_report, fs::Save};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Detectors.
pub enum Detector {
    /// Spectrometer.
    Spectrometer(Histogram),
}

impl Save for Detector {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
        match *self {
            Self::Spectrometer(ref hist) => {
                hist.save(&path.with_extension("csv"))?;
            }
        }

        Ok(())
    }
}

impl Display for Detector {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Spectrometer(ref hist) => {
                fmt_report!(fmt, "Spectrometer", "kind");
                fmt_report!(fmt, hist, "histogram");
                Ok(())
            }
        }
    }
}
