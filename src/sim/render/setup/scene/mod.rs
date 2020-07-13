//! Scene settings structure.

pub mod lighting;

pub use self::lighting::*;

use crate::{access, display_field, display_field_ln, form::Camera};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Scene settings structure.
#[load]
pub struct Scene {
    /// Engine selection.
    cam: Camera,
    /// Lighting properties.
    lighting: Lighting,
}

impl Scene {
    access!(cam, Camera);
    access!(lighting, Lighting);
}

impl Display for Scene {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "camera", &self.cam)?;
        display_field!(fmt, "lighting", &self.lighting)
    }
}
