//! Shader settings.

use nalgebra::Point3;
use palette::{Gradient, LinSrgba};

/// Shader settings.
pub struct Shader<'a> {
    /// Sun position (m).
    pub sun_pos: Point3<f64>,
    /// Ambient, diffuse, and occlusion lighting fractions.
    pub light: [f64; 3],
    /// Ambient, diffuse, and occlusion shadowing fractions.
    pub shadow: [f64; 2],
    /// Ambient lighting fraction.
    pub spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    pub occ_dist: [f64; 2],
    /// Effect fall-off rate.
    pub fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius (rad).
    pub soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of ambient shadowing samples and the scaling power.
    pub ambient_shadow_samples: Option<(i32, i32)>,
    /// Sky colour gradient.
    pub sky_grad: &'a Gradient<LinSrgba>,
    /// Data colouring gradient.
    pub data_grad: &'a Gradient<LinSrgba>,
}

impl<'a> Shader<'a> {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments, clippy::cognitive_complexity)]
    #[inline]
    #[must_use]
    pub fn new(
        sun_pos: Point3<f64>,
        light: [f64; 3],
        shadow: [f64; 2],
        spec_pow: i32,
        occ_dist: [f64; 2],
        fall_off: f64,
        soft_shadow_samples: Option<(i32, f64)>,
        ambient_shadow_samples: Option<(i32, i32)>,
        sky_grad: &'a Gradient<LinSrgba>,
        data_grad: &'a Gradient<LinSrgba>,
    ) -> Self {
        debug_assert!(light[0] > 0.0);
        debug_assert!(light[1] > 0.0);
        debug_assert!(light[2] > 0.0);
        debug_assert!(shadow[0] > 0.0);
        debug_assert!(shadow[1] > 0.0);
        debug_assert!(spec_pow > 0);
        debug_assert!(occ_dist[0] > 0.0);
        debug_assert!(occ_dist[1] > 0.0);
        debug_assert!(fall_off > 0.0);
        debug_assert!(
            soft_shadow_samples.is_none()
                || soft_shadow_samples
                    .expect("Number of soft shadow samples must be greater than one.")
                    .0
                    > 1
        );
        debug_assert!(
            soft_shadow_samples.is_none()
                || soft_shadow_samples
                    .expect("Soft shadow numerical aperture must be greater than zero.")
                    .1
                    > 0.0
        );
        debug_assert!(
            ambient_shadow_samples.is_none()
                || ambient_shadow_samples
                    .expect("Number of ambient shadow samples must be greater than one.")
                    .0
                    > 1
        );
        debug_assert!(
            ambient_shadow_samples.is_none()
                || ambient_shadow_samples
                    .expect("Ambient shadow numerical aperture must be greater than zero.")
                    .1
                    > 0
        );

        let light_total = light[0] + light[1] + light[2];
        let shadow_total = shadow[0] + shadow[1];

        Self {
            sun_pos,
            light: [
                light[0] / light_total,
                light[1] / light_total,
                light[2] / light_total,
            ],
            shadow: [shadow[0] / shadow_total, shadow[1] / shadow_total],
            spec_pow,
            occ_dist,
            fall_off,
            soft_shadow_samples,
            ambient_shadow_samples,
            sky_grad,
            data_grad,
        }
    }
}
