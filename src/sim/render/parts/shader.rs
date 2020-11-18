//! Shader settings.

use crate::{access, math::Vec3};

/// Colouring settings.
pub struct Shader {
    /// Sun position used for lighting calculations [m].
    sun_pos: Vec3,
}

impl Shader {
    access!(sun_pos, Vec3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sun_pos: Vec3) -> Self {
        Self { sun_pos }
    }
}
