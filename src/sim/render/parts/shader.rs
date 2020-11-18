//! Shader settings.

use crate::{access, math::Vec3};

/// Colouring settings.
pub struct Settings<'a> {
    /// Sun position used for lighting calculations.
    sun_pos: Vec3,
}

impl<'a> Settings<'a> {
    access!(sun_pos, Vec3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(sun_pos: Vec3) -> Self {
        Self { sun_pos }
    }
}
