//! Diffusion simulation input structure.

use crate::{diffusion::Settings, display_field, display_field_ln};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Formatter, Result};

/// Diffusion main input structure.
pub struct Scene<'a> {
    /// Engine settings.
    pub sett: &'a Settings,
    /// Diffusion coefficents.
    pub coeffs: &'a Array3<f64>,
}

impl<'a> Scene<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sett: &'a Settings, coeffs: &'a Array3<f64>) -> Self {
        Self { sett, coeffs }
    }
}

impl<'a> Display for Scene<'a> {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "settings", &self.sett)?;
        display_field!(
            fmt,
            "maximum coefficients",
            self.coeffs
                .max()
                .expect("Failed to determine maximum diffusion coefficient."),
            "m^2/s"
        )
    }
}
