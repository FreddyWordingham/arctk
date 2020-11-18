//! Shader settings.

use crate::{access, clone, math::Pos3};

/// Colouring settings.
pub struct Shader {
    /// Sun position used for lighting calculations [m].
    sun_pos: Pos3,
    // Ambient lighting fraction.
    light_ambient_frac: f64,
    // Ambient lighting fraction.
    light_diffuse_frac: f64,
    // Ambient lighting fraction.
    light_specular_frac: f64,
    // Ambient lighting fraction.
    spec_pow: i32,
}

impl Shader {
    access!(sun_pos, Pos3);
    clone!(light_ambient_frac, f64);
    clone!(light_diffuse_frac, f64);
    clone!(light_specular_frac, f64);
    clone!(spec_pow, i32);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        sun_pos: Pos3,
        light_ambient_frac: f64,
        light_diffuse_frac: f64,
        light_specular_frac: f64,
        spec_pow: i32,
    ) -> Self {
        debug_assert!(light_ambient_frac > 0.0);
        debug_assert!(light_diffuse_frac > 0.0);
        debug_assert!(light_specular_frac > 0.0);
        debug_assert!(spec_pow > 0);

        let total_light = light_ambient_frac + light_diffuse_frac + light_specular_frac;
        let light_ambient_frac = light_ambient_frac / total_light;
        let light_diffuse_frac = light_diffuse_frac / total_light;
        let light_specular_frac = light_specular_frac / total_light;

        Self {
            sun_pos,
            light_ambient_frac,
            light_diffuse_frac,
            light_specular_frac,
            spec_pow,
        }
    }
}
