//! Scene builder structure.

use crate::{
    display_field, display_field_ln,
    form::{Camera, Lighting},
    Build, Error,
};
use attr::load;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// Scene settings structure.
#[load]
pub struct Scene {
    /// Camera form.
    cam: Camera,
    /// Lighting properties.
    lighting: Lighting,
}

impl Build for Scene {
    type Inst = crate::render::Scene;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.cam.build(in_dir)?,
            self.lighting.build(in_dir)?,
        ))
    }
}

impl Display for Scene {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "camera", &self.cam)?;
        display_field!(fmt, "lighting", &self.lighting)
    }
}
