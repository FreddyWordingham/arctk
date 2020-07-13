//! Scene settings structure.

pub mod lighting;

pub use self::lighting::*;

use crate::{access, display_field, display_field_ln, render::Camera};
use std::fmt::{Display, Formatter, Result};

/// Scene settings structure.
pub struct Scene {
    /// Camera.
    cam: Camera,
    /// Lighting properties.
    lighting: Lighting,
}

impl Scene {
    access!(cam, Camera);
    access!(lighting, Lighting);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(cam: Camera, lighting: Lighting) -> Self {
        Self { cam, lighting }
    }
}

impl Display for Scene {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "camera", &self.cam)?;
        display_field!(fmt, "lighting", &self.lighting)
    }
}
