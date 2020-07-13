//! Lighting builder structure.

use crate::{
    display_field, display_field_ln,
    form::Sky,
    render::{Light, Samples, Shadow},
    Build, Error,
};
use attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Conglomerate lighting setup structure.
#[load]
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

impl Build for Lighting {
    type Inst = crate::render::Lighting;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.sky.build(in_dir)?,
            self.samples,
            self.light,
            self.shadow,
        ))
    }
}

impl Display for Lighting {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        display_field_ln!(fmt, "sky", &self.sky)?;
        display_field_ln!(fmt, "sampling", &self.samples)?;
        display_field_ln!(fmt, "light", &self.light)?;
        display_field!(fmt, "shadow", &self.shadow)
    }
}
