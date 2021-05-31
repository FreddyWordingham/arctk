//! Datacube formatting functions.

use crate::{
    fmt_report,
    ord::{X, Y, Z},
    util::fmt::Analyze,
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Error, Formatter};

/// Three-dimensional array analysis structure.
pub struct DataCube {
    /// Resolution.
    res: [usize; 3],
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

impl DataCube {
    /// Construct a new instance.
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    pub fn new(data: &Array3<f64>) -> Self {
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
            // TODO: Add to other data_ methods when stabilised.
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
            res: [shape[X], shape[Y], shape[Z]],
            sum: data.sum(),
            min,
            max,
            ave,
            sd,
        }
    }
}

impl Display for DataCube {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!("[{} x {} x {}]", self.res[X], self.res[Y], self.res[Z]),
            "resolution"
        );
        fmt_report!(fmt, self.res[X] * self.res[Y] * self.res[Z], "length");
        fmt_report!(fmt, self.sum, "sum");
        fmt_report!(fmt, self.min, "minimum value");
        fmt_report!(fmt, self.max, "maximum value");
        fmt_report!(fmt, self.ave, "average value");
        fmt_report!(fmt, self.sd, "standard deviation");
        Ok(())
    }
}

impl Analyze for Array3<f64> {
    type Inst = DataCube;

    #[inline]
    #[must_use]
    fn display(&self) -> Self::Inst {
        Self::Inst::new(self)
    }
}
