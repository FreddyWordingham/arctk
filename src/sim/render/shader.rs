//! Shader settings.

use crate::{access, clone, fmt_report, img::Gradient, math::Pos3, util::gradient::to_string};
use std::fmt::{Display, Error, Formatter};

/// Colouring settings.
pub struct Shader<'a> {
    /// Sun position used for lighting calculations (m).
    sun_pos: Pos3,
    /// Ambient, diffuse, and occlusion lighting fractions.
    light: [f64; 3],
    /// Ambient, diffuse, and occlusion shadowing fractions.
    shadow: [f64; 2],
    /// Ambient lighting fraction.
    spec_pow: i32,
    /// Lighting and shadowing occlusion testing distances.
    occ_dist: [f64; 2],
    /// Effect fall-off rate.
    fall_off: f64,
    /// Optional number of soft shadowing samples, and angular radius (rad).
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of ambient shadowing samples and the scaling power.
    ambient_shadow_samples: Option<(i32, i32)>,
    /// Sky colour gradient.
    sky_grad: &'a Gradient,
    /// Data colouring gradient.
    data_grad: &'a Gradient,
}

impl<'a> Shader<'a> {
    access!(sun_pos, Pos3);
    access!(light, [f64; 3]);
    access!(shadow, [f64; 2]);
    clone!(spec_pow, i32);
    access!(occ_dist, [f64; 2]);
    clone!(fall_off, f64);
    clone!(soft_shadow_samples, Option<(i32, f64)>);
    clone!(ambient_shadow_samples, Option<(i32, i32)>);
    access!(sky_grad, Gradient);
    access!(data_grad, Gradient);

    /// Construct a new instance.
    #[allow(clippy::cognitive_complexity)]
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        sun_pos: Pos3,
        light: [f64; 3],
        shadow: [f64; 2],
        spec_pow: i32,
        occ_dist: [f64; 2],
        fall_off: f64,
        soft_shadow_samples: Option<(i32, f64)>,
        ambient_shadow_samples: Option<(i32, i32)>,
        sky_grad: &'a Gradient,
        data_grad: &'a Gradient,
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
        debug_assert!(soft_shadow_samples.is_none() || soft_shadow_samples.unwrap().0 > 1);
        debug_assert!(soft_shadow_samples.is_none() || soft_shadow_samples.unwrap().1 > 0.0);
        debug_assert!(ambient_shadow_samples.is_none() || ambient_shadow_samples.unwrap().0 > 1);
        debug_assert!(ambient_shadow_samples.is_none() || ambient_shadow_samples.unwrap().1 > 0);

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

impl Display for Shader<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!("{}, {}, {}", self.sun_pos.x, self.sun_pos.y, self.sun_pos.z),
            "sun position (m)"
        );
        fmt_report!(
            fmt,
            &format!("[{}, {}, {}]", self.light[0], self.light[1], self.light[2]),
            "lighting fractions"
        );
        fmt_report!(
            fmt,
            &format!("[{}, {}]", self.shadow[0], self.shadow[1]),
            "shadowing fractions"
        );
        fmt_report!(fmt, self.spec_pow, "specular power");
        fmt_report!(
            fmt,
            &format!("[{}, {}]", self.occ_dist[0], self.occ_dist[1]),
            "occlusion distance (m)"
        );
        fmt_report!(fmt, self.fall_off, "fall off rate (m^-1)");

        let soft_shadow_samples = if let Some((n, alpha)) = self.soft_shadow_samples {
            format!("{} samples, angle (deg) {}", n, alpha.to_degrees())
        } else {
            "OFF".to_string()
        };
        fmt_report!(fmt, soft_shadow_samples, "soft shadowing");

        let ambient_shadow_samples = if let Some((n, p)) = self.ambient_shadow_samples {
            format!("{} samples, power {}", n, p)
        } else {
            "OFF".to_string()
        };
        fmt_report!(fmt, ambient_shadow_samples, "ambient shadowing");
        fmt_report!(fmt, to_string(self.sky_grad, 32), "sky gradient");
        fmt_report!(fmt, to_string(self.data_grad, 32), "data gradient");
        Ok(())
    }
}
