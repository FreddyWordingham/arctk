//! Datacube formatting functions.

use crate::{
    fmt_report,
    ord::{X, Y},
    util::fmt::Analyze,
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Error, Formatter};

/// Two-dimensional array analysis structure.
pub struct DataSquare {
    /// Resolution.
    res: [usize; 2],
    /// Sum of values.
    sum: f64,
    /// Minimum value.
    min: f64,
    /// Maximum value.
    max: f64,
    /// Average value.
    ave: f64,
    /// Standard deviation.
    sd: f64,
}

impl DataSquare {
    /// Construct a new instance.
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    pub fn new(data: &Array2<f64>) -> Self {
        let shape = data.shape();

        let ave = data.sum() / data.len() as f64;
        let sd = (data
            .map(|v| {
                let d = ave - v;
                d * d
            })
            .sum()
            / data.len() as f64)
            .sqrt();

        let min = if let Ok(x) = data.min() {
            *x
        } else {
            println!("[WARN] Could not determine minimum value of datacube.");
            0.0
        };
        let max = if let Ok(x) = data.max() {
            *x
        } else {
            println!("[WARN] Could not determine maximum value of datacube.");
            0.0
        };

        Self {
            res: [shape[X], shape[Y]],
            sum: data.sum(),
            min,
            max,
            ave,
            sd,
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
        fmt_report!(fmt, self.res[X] * self.res[Y], "length");
        fmt_report!(fmt, self.sum, "sum");
        fmt_report!(fmt, self.min, "minimum value");
        fmt_report!(fmt, self.max, "maximum value");
        fmt_report!(fmt, self.ave, "average value");
        fmt_report!(fmt, self.sd, "standard deviation");
        Ok(())
    }
}

impl Analyze for Array2<f64> {
    type Inst = DataSquare;

    #[inline]
    #[must_use]
    fn display(&self) -> Self::Inst {
        Self::Inst::new(self)
    }
}
