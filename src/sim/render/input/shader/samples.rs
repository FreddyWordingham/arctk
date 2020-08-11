//! Lighting samples setup structure.

use crate::{clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Lighting structure.
#[load]
pub struct Samples {
    /// Optional number of ambient occlusion samples.
    ambient_occlusion: Option<i32>,
    /// Optional number of soft shadow samples.
    soft_shadows: Option<i32>,
}

impl Samples {
    clone!(ambient_occlusion, Option<i32>);
    clone!(soft_shadows, Option<i32>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ambient_occlusion: Option<i32>, soft_shadows: Option<i32>) -> Self {
        debug_assert!(ambient_occlusion.is_none() || ambient_occlusion.unwrap() > 1);
        debug_assert!(soft_shadows.is_none() || soft_shadows.unwrap() > 1);

        Self {
            ambient_occlusion,
            soft_shadows,
        }
    }
}

impl Display for Samples {
    #[allow(clippy::expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        if let Some(ambient_occlusion) = self.ambient_occlusion {
            display_field_ln!(fmt, "ambient occlusion", ambient_occlusion)?;
        } else {
            display_field_ln!(fmt, "ambient occlusion", "[OFF]")?;
        }
        if let Some(soft_shadows) = self.soft_shadows {
            display_field!(fmt, "soft shadow", soft_shadows)
        } else {
            display_field!(fmt, "soft shadows", "[OFF]")
        }
    }
}
