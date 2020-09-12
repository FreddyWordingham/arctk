//! Light setup structure.

use crate::{clone, display_field, display_field_ln};
use arctk_attr::load;
use std::fmt::{Display, Formatter, Result};

/// Lighting structure.
#[load]
pub struct Light {
    /// Ambient lighting fraction.
    ambient: f64,
    /// Diffuse lighting fraction.
    diffuse: f64,
    /// Specular lighting fraction.
    specular: f64,
    /// Specular lighting power.
    spec_pow: i32,
}

impl Light {
    clone!(ambient, f64);
    clone!(diffuse, f64);
    clone!(specular, f64);
    clone!(spec_pow, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ambient: f64, diffuse: f64, specular: f64, spec_pow: i32) -> Self {
        Self {
            ambient,
            diffuse,
            specular,
            spec_pow,
        }
    }
}

impl Display for Light {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "ambient fraction", self.ambient)?;
        display_field_ln!(fmt, "diffuse fraction", self.diffuse)?;
        display_field_ln!(fmt, "specular fraction", self.specular)?;
        display_field!(fmt, "specular power", self.spec_pow)
    }
}
