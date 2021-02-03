//! Datacube formatting functions.

use crate::{fmt_report, util::fmt::Analyze};
use ndarray::Array4;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Error, Formatter};

/// Four-dimensional array analysis structure.
pub struct DataTesseract {
    /// Resolution.
    res: [usize; 4],
    /// Minimum value.
    min: f64,
    /// Maximum value.
    max: f64,
    /// Average value.
    ave: f64,
    /// Standard deviation.
    sd: f64,
}

impl DataTesseract {
    /// Construct a new instance.
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    pub fn new(data: &Array4<f64>) -> Self {
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

        Self {
            res: [shape[0], shape[1], shape[2], shape[3]],
            min: *data.min().expect("Unable to determine minimum value."),
            max: *data.max().expect("Unable to determine maximum value."),
            ave,
            sd,
        }
    }
}

impl Display for DataTesseract {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!(
                "[{} x {} x {} x {}]",
                self.res[0], self.res[1], self.res[2], self.res[3]
            ),
            "resolution"
        );
        fmt_report!(
            fmt,
            self.res[0] * self.res[1] * self.res[2] * self.res[3],
            "length"
        );
        fmt_report!(fmt, self.min, "minimum value");
        fmt_report!(fmt, self.max, "maximum value");
        fmt_report!(fmt, self.ave, "average value");
        fmt_report!(fmt, self.sd, "standard deviation");
        Ok(())
    }
}

impl Analyze for Array4<f64> {
    type Inst = DataTesseract;

    #[inline]
    #[must_use]
    fn display(&self) -> Self::Inst {
        Self::Inst::new(self)
    }
}
