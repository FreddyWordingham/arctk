//! Lighting setup module.

pub mod light;
pub mod samples;
pub mod shadow;
pub mod sky;

pub use self::{light::*, samples::*, shadow::*, sky::*};

/// Conglomerate lighting setup structure.
use crate::{access, display_field, display_field_ln};
use std::fmt::{Display, Formatter, Result};

/// Lighting structure.
#[derive(Debug)]
pub struct Lighting {
    /// Sky settings.
    sky: Sky,
    /// Lighting samples.
    samples: Samples,
    /// Lighting settings.
    light: Light,
    /// Shadowing settings.
    shadow: Shadow,
}

impl Lighting {
    access!(sky, Sky);
    access!(samples, Samples);
    access!(light, Light);
    access!(shadow, Shadow);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sky: Sky, samples: Samples, light: Light, shadow: Shadow) -> Self {
        Self {
            sky,
            samples,
            light,
            shadow,
        }
    }
}

impl Display for Lighting {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "sky", &self.sky)?;
        display_field_ln!(fmt, "sampling", &self.samples)?;
        display_field_ln!(fmt, "light", &self.light)?;
        display_field!(fmt, "shadow", &self.shadow)
    }
}
