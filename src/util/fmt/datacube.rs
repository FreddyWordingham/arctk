//! Datacube formatting functions.

use crate::{
    fmt_report,
    ord::{X, Y, Z},
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::fmt::Formatter;

/// Write a datacubes properties to a given formatter.
/// # Errors
/// if the minimum value of the data array can not be determined or
/// if the maximum value of the data array can not be determined.
#[inline]
pub fn display_datacube(
    fmt: &mut Formatter,
    datacube: &Array3<f64>,
) -> Result<(), std::fmt::Error> {
    writeln!(fmt, "...")?;
    let res = datacube.shape();
    fmt_report!(
        fmt,
        &format!("[{}x{}x{}]", res[X], res[Y], res[Z]),
        "resolution"
    );
    fmt_report!(
        fmt,
        datacube
            .min()
            .unwrap_or_else(|_| panic!("Failed to determine minimum value.")),
        "minimum"
    );
    fmt_report!(
        fmt,
        datacube
            .max()
            .unwrap_or_else(|_| panic!("Failed to determine maximum value.")),
        "maximum"
    );
    let sum = datacube.sum();
    fmt_report!(fmt, sum / datacube.len() as f64, "average");
    fmt_report!(fmt, sum, "sum");
    Ok(())
}
