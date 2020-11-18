//! Lighting calculation.

use crate::sim::render::Shader;
use crate::{
    geom::{Hit, Ray},
    math::Dir3,
    phys::Crossing,
};

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn light<T>(input: &Input, cam_pos: &Pos3, ray: &Ray, norm: &Vec3) -> f64 {
    let light_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(cam_pos() - ray.pos());
    let ref_dir = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());

    let mut ambient = 1.0;
    let mut diffuse = hit.side().norm().dot(&light_dir).max(0.0);
    let mut specular = view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(shader.light().spec_pow());

    ambient *= shader.light().ambient();
    diffuse *= shader.light().diffuse();
    specular *= shader.light().specular();

    ambient + diffuse + specular
}
