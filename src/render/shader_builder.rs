//! Shader configration.

use nalgebra::Point3;
use palette::{Gradient, LinSrgba};
use serde::Deserialize;
use std::collections::HashMap;

use crate::render::Shader;

/// Aesthetic settings.
#[derive(Deserialize)]
pub struct ShaderBuilder {
    /// Sun position (m).
    sun_pos: Point3<f64>,
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
    /// Optional number of soft shadowing samples, and angular radius (deg).
    soft_shadow_samples: Option<(i32, f64)>,
    /// Optional number of ambient shadowing samples and the scaling power.
    ambient_shadow_samples: Option<(i32, i32)>,
    /// Sky colour gradient.
    sky_grad: String,
    /// Data colouring gradient.
    data_grad: String,
}

impl ShaderBuilder {
    /// Get the names of the `Gradient`s used.
    #[inline]
    #[must_use]
    pub fn used_gradient_names(&self) -> Vec<String> {
        vec![self.sky_grad.clone(), self.data_grad.clone()]
    }
}

impl<'a> ShaderBuilder {
    /// Build the `Shader`.
    #[inline]
    #[must_use]
    pub fn build(&self, grads: &'a HashMap<String, Gradient<LinSrgba>>) -> Shader<'a> {
        let soft_shadow_samples = if let Some((n, alpha)) = self.soft_shadow_samples {
            Some((n, alpha.to_radians()))
        } else {
            None
        };

        Shader::new(
            self.sun_pos,
            self.light,
            self.shadow,
            self.spec_pow,
            self.occ_dist,
            self.fall_off,
            soft_shadow_samples,
            self.ambient_shadow_samples,
            grads.get(&self.sky_grad).unwrap_or_else(|| {
                panic!("Failed to link shader-gradient key: {}", &self.sky_grad)
            }),
            grads.get(&self.data_grad).unwrap_or_else(|| {
                panic!("Failed to link shader-gradient key: {}", &self.data_grad)
            }),
        )
    }
}
