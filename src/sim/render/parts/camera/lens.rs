//! Camera lens implementation.

use crate::{clone, display_field, display_field_ln, X, Y};
use std::fmt::{Display, Formatter, Result};

/// Lens structure.
#[derive(Debug)]
pub struct Lens {
    /// Swivel to apply after targeting [rad].
    swivel: [f64; 2],
    /// Field of view [rad].
    fov: f64,
}

impl Lens {
    clone!(swivel, [f64; 2]);
    clone!(fov, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(swivel: [f64; 2], fov: f64) -> Self {
        debug_assert!(fov > 0.0);

        Self { swivel, fov }
    }
}

impl Display for Lens {
    #[allow(clippy::expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(
            fmt,
            "swivel",
            &format!(
                "{} : {}",
                self.swivel[X].to_degrees(),
                self.swivel[Y].to_degrees()
            ),
            "deg"
        )?;
        display_field!(fmt, "field of view", self.fov.to_degrees(), "deg")
    }
}
