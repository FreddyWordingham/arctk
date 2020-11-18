//! Loadable shader settings.

use crate::{
    err::Error,
    file::Build,
    math::Vec3,
    ord::{X, Y, Z},
    sim::render::Shader,
};
use arctk_attr::load;
use std::path::Path;

/// Colouring settings builder.
#[load]
pub struct ShaderBuilder {
    /// Sun position used for lighting calculations [m].
    sun_pos: [f64; 3],
}

impl Build for ShaderBuilder {
    type Inst = Shader;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(Vec3::new(
            self.sun_pos[X],
            self.sun_pos[Y],
            self.sun_pos[Z],
        )))
    }
}
