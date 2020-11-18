//! Loadable shader settings.

use crate::{
    err::Error,
    file::Build,
    math::Pos3,
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
    /// Relative ambient, diffuse, and occlusion lighting powers.
    light: [f64; 3],
    /// Relative ambient, diffuse, and occlusion shadowing powers.
    shadow: [f64; 3],
    /// Ambient lighting fraction.
    spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    occ_dist: [f64; 2],
}

impl Build for ShaderBuilder {
    type Inst = Shader;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            Pos3::new(self.sun_pos[X], self.sun_pos[Y], self.sun_pos[Z]),
            self.light,
            self.shadow,
            self.spec_pow,
            self.occ_dist,
        ))
    }
}
