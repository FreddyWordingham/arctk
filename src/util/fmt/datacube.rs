//! Datacube formatting functions.

use crate::{
    fmt_report,
    ord::{X, Y, Z},
};
use ndarray::{Array2, Array3, Array4};
use ndarray_stats::QuantileExt;
use std::fmt::Formatter;

/// Write a datasquares properties to a given formatter.
/// # Errors
/// if the minimum value of the data array can not be determined or
/// if the maximum value of the data array can not be determined.
#[inline]
pub fn display_datasquare(
    fmt: &mut Formatter,
    datasquare: &Array2<f64>,
) -> Result<(), std::fmt::Error> {
    writeln!(fmt, "...")?;
    let res = datasquare.shape();
    fmt_report!(fmt, &format!("[{} x {}]", res[X], res[Y]), "resolution");
    fmt_report!(
        fmt,
        datasquare
            .min()
            .unwrap_or_else(|_| panic!("Failed to determine minimum value.")),
        "minimum"
    );
    fmt_report!(
        fmt,
        datasquare
            .max()
            .unwrap_or_else(|_| panic!("Failed to determine maximum value.")),
        "maximum"
    );
    let sum = datasquare.sum();
    fmt_report!(fmt, sum / datasquare.len() as f64, "average");
    fmt_report!(fmt, sum, "sum");
    Ok(())
}

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
        &format!("[{} x {} x {}]", res[X], res[Y], res[Z]),
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

/// Write a datatesseracts properties to a given formatter.
/// # Errors
/// if the minimum value of the data array can not be determined or
/// if the maximum value of the data array can not be determined.
#[inline]
pub fn display_datatesseract(
    fmt: &mut Formatter,
    datacube: &Array4<f64>,
) -> Result<(), std::fmt::Error> {
    writeln!(fmt, "...")?;
    let res = datacube.shape();
    fmt_report!(
        fmt,
        &format!("[{} x {} x {} x {}]", res[0], res[1], res[2], res[3]),
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
