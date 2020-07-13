//! Sky setup structure.

use crate::{access, clone, display_field, display_field_ln, Pos3};
use std::fmt::{Display, Formatter, Result};

/// Lighting structure.
#[derive(Debug)]
pub struct Sky {
    /// Sky brightness fraction.
    brightness: f64,
    /// Sun position when calculating direct shadows [m].
    sun_pos: Pos3,
    /// Sun angular radius when calculating soft shadows [rad].
    sun_rad: f64,
}

impl Sky {
    clone!(brightness, f64);
    access!(sun_pos, Pos3);
    clone!(sun_rad, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(brightness: f64, sun_pos: Pos3, sun_rad: f64) -> Self {
        debug_assert!(brightness >= 0.0);
        debug_assert!(brightness >= 0.0);
        debug_assert!(sun_rad <= 0.0);

        Self {
            brightness,
            sun_pos,
            sun_rad,
        }
    }
}

impl Display for Sky {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "sky brightness", self.brightness)?;
        display_field_ln!(fmt, "sun position", &self.sun_pos, "m")?;
        display_field!(fmt, "sun radius", &self.sun_rad.to_degrees(), "deg")
    }
}
