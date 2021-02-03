//! Shader settings.

use crate::{
    err::Error,
    fmt_report,
    img::Gradient,
    math::Pos3,
    ord::{Link, Name, Set, X, Y, Z},
    sim::render::Shader,
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Colouring settings.
#[file]
pub struct ShaderLinker {
    /// Sun position used for lighting calculations (m).
    sun_pos: [f64; 3],
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
    sky_grad: Name,
    /// Data colouring gradient.
    data_grad: Name,
}

impl<'a> Link<'a, Gradient> for ShaderLinker {
    type Inst = Shader<'a>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        vec![self.sky_grad.clone(), self.data_grad.clone()]
    }

    #[inline]
    fn link(self, reg: &'a mut Set<Gradient>) -> Result<Self::Inst, Error> {
        let soft_shadow_samples = if let Some((n, alpha)) = self.soft_shadow_samples {
            Some((n, alpha.to_radians()))
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
            reg.get(&self.sky_grad).unwrap_or_else(|| {
                panic!("Failed to link shader-gradient key: {}", &self.sky_grad)
            }),
            reg.get(&self.data_grad).unwrap_or_else(|| {
                panic!("Failed to link shader-gradient key: {}", &self.data_grad)
            }),
        ))
    }
}

impl Display for ShaderLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!(
                "({}, {}, {})",
                self.sun_pos[X], self.sun_pos[Y], self.sun_pos[Z]
            ),
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
            format!("{} samples, angle (deg) {}", n, alpha)
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
        fmt_report!(fmt, &format!("{}", self.sky_grad), "sky gradient");
        fmt_report!(fmt, &format!("{}", self.data_grad), "data gradient");
        Ok(())
    }
}
