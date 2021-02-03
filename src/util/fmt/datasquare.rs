//! Datacube formatting functions.

use crate::{
    fmt_report,
    ord::{X, Y},
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Error, Formatter};

/// Array2<f64> analysis structure.
pub struct DataSquare {
    /// Resolution.
    res: [usize; 2],
    /// Minimum value.
    min: f64,
    /// Maximum value.
    max: f64,
}

impl DataSquare {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(data: &Array2<f64>) -> Self {
        let shape = data.shape();

        Self {
            res: [shape[X], shape[Y]],
            min: *data.min().expect("Unable to determine minimum value."),
            max: *data.max().expect("Unable to determine maximum value."),
        }
    }
}

impl Display for DataSquare {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!("[{} x {}]", self.res[X], self.res[Y]),
            "resolution"
        );
        fmt_report!(fmt, self.min, "minimum value");
        fmt_report!(fmt, self.max, "maximum value");
        Ok(())
    }
}
