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
    /// Effect fall-off rate.
    fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius [deg].
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of soft shadowing samples.
    ambient_shadow_samples: Option<i32>,
}

impl Build for ShaderBuilder {
    type Inst = Shader;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        let soft_shadow_samples = if let Some((samples, rad)) = self.soft_shadow_samples {
            Some((samples, rad.to_radians()))
        } else {
            None
        };

        Ok(Self::Inst::new(
            Pos3::new(self.sun_pos[X], self.sun_pos[Y], self.sun_pos[Z]),
            self.light,
            self.shadow,
            self.spec_pow,
            self.occ_dist,
            self.fall_off,
            soft_shadow_samples,
            self.ambient_shadow_samples,
        ))
    }
}
