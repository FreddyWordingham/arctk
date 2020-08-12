//! Shadow setup structure.

use crate::{access, clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Lighting structure.
#[load]
pub struct Shadow {
    /// Ambient shadowing fraction.
    ambient: f64,
    /// Direct shadowing fraction.
    direct: f64,
    /// Ambient occlusion power.
    ao_pow: i32,
}

impl Shadow {
    access!(ambient, f64);
    access!(direct, f64);
    clone!(ao_pow, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ambient: f64, direct: f64, ao_pow: i32) -> Self {
        Self {
            ambient,
            direct,
            ao_pow,
        }
    }
}

impl Display for Shadow {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "ambient fraction", self.ambient)?;
        display_field_ln!(fmt, "direct fraction", self.direct)?;
        display_field!(fmt, "ambient occlusion power", self.ao_pow)
    }
}
