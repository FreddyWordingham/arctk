//! Lighting calculation.

use crate::sim::render::Input;
use crate::{
    geom::Ray,
    math::{Dir3, Pos3},
    phys::Crossing,
};

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn lighting(input: &Input, cam_pos: &Pos3, ray: &Ray, norm: &Dir3) -> f64 {
    let light_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(cam_pos - ray.pos());
    let ref_dir = Crossing::calc_ref_dir(ray.dir(), norm);

    let shader = input.shader;

    let [ambient, mut diffuse, mut specular] = shader.light();
    diffuse *= norm.dot(&light_dir).max(0.0);
    specular *= view_dir.dot(&ref_dir).max(0.0).powi(shader.spec_pow());

    ambient + diffuse + specular
}
