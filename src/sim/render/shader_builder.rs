//! Shader form.

use crate::{
    access, display_field, display_field_ln,
    render::{CameraBuilder, Light, Samples, Shader, Shadow, SkyBuilder},
    Build, Error,
};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable light and shadow settings.
#[load]
pub struct ShaderBuilder {
    /// Sky builder.
    sky: SkyBuilder,
    /// Lighting samples.
    samples: Samples,
    /// Lighting settings.
    light: Light,
    /// Shadowing settings.
    shadow: Shadow,
    /// Camera builder.
    cam: CameraBuilder,
}

impl ShaderBuilder {
    access!(sky, SkyBuilder);
    access!(samples, Samples);
    access!(light, Light);
    access!(shadow, Shadow);
    access!(cam, CameraBuilder);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        sky: SkyBuilder,
        samples: Samples,
        light: Light,
        shadow: Shadow,
        cam: CameraBuilder,
    ) -> Self {
        Self {
            sky,
            samples,
            light,
            shadow,
            cam,
        }
    }
}

impl Build for ShaderBuilder {
    type Inst = Shader;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.sky.build(in_dir)?,
            self.samples,
            self.light,
            self.shadow,
            self.cam.build(in_dir)?,
        ))
    }
}

impl Display for ShaderBuilder {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "sky", &self.sky)?;
        display_field_ln!(fmt, "sampling", &self.samples)?;
        display_field_ln!(fmt, "light", &self.light)?;
        display_field_ln!(fmt, "shadow", &self.shadow)?;
        display_field!(fmt, "camera", &self.cam)
    }
}
