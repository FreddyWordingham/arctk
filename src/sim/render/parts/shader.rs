//! Shader settings.

use crate::{access, clone, math::Pos3};

/// Colouring settings.
pub struct Shader {
    /// Sun position used for lighting calculations [m].
    sun_pos: Pos3,
    /// Ambient, diffuse, and occlusion lighting fractions.
    light: [f64; 3],
    /// Ambient, diffuse, and occlusion shadowing fractions.
    shadow: [f64; 3],
    /// Ambient lighting fraction.
    spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    occ_dist: [f64; 2],
    /// Effect fall-off rate.
    fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius [rad].
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of soft shadowing samples.
    ambient_shadow_samples: Option<i32>,
}

impl Shader {
    access!(sun_pos, Pos3);
    access!(light, [f64; 3]);
    access!(shadow, [f64; 3]);
    clone!(spec_pow, i32);
    access!(occ_dist, [f64; 2]);
    clone!(fall_off, f64);
    access!(soft_shadow_samples, Option<(i32, f64)>);
    access!(ambient_shadow_samples, Option<i32>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        sun_pos: Pos3,
        light: [f64; 3],
        shadow: [f64; 3],
        spec_pow: i32,
        occ_dist: [f64; 2],
        fall_off: f64,
        soft_shadow_samples: Option<(i32, f64)>,
        ambient_shadow_samples: Option<i32>,
    ) -> Self {
        debug_assert!(light[0] > 0.0);
        debug_assert!(light[1] > 0.0);
        debug_assert!(light[2] > 0.0);
        debug_assert!(shadow[0] > 0.0);
        debug_assert!(shadow[1] > 0.0);
        debug_assert!(shadow[2] > 0.0);
        debug_assert!(spec_pow > 0);
        debug_assert!(occ_dist[0] > 0.0);
        debug_assert!(occ_dist[1] > 0.0);
        debug_assert!(fall_off > 0.0);
        debug_assert!(soft_shadow_samples.is_none() || soft_shadow_samples.unwrap().0 > 1);
        debug_assert!(soft_shadow_samples.is_none() || soft_shadow_samples.unwrap().1 > 0.0);
        debug_assert!(ambient_shadow_samples.is_none() || ambient_shadow_samples.unwrap() > 1);

        let light_total = light[0] + light[1] + light[2];
        let shadow_total = shadow[0] + shadow[1] + shadow[2];

        Self {
            sun_pos,
            light: [
                light[0] / light_total,
                light[1] / light_total,
                light[2] / light_total,
            ],
            shadow: [
                shadow[0] / shadow_total,
                shadow[1] / shadow_total,
                shadow[2] / shadow_total,
            ],
            spec_pow,
            occ_dist,
            fall_off,
            soft_shadow_samples,
            ambient_shadow_samples,
        }
    }
}
