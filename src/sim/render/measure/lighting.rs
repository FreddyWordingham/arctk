//! Lighting calculation.

use crate::sim::render::{occlusion, Input};
use crate::{geom::Ray, math::Dir3, phys::Crossing};

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn lighting(input: &Input, ray: &Ray, norm: &Dir3) -> f64 {
    let light_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(input.cam.pos() - ray.pos());
    let ref_dir = Crossing::calc_ref_dir(ray.dir(), norm);

    let [ambient, mut diffuse, mut specular] = input.shader.light();
    diffuse *= norm.dot(&light_dir).max(0.0);
    specular *= view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(input.shader.spec_pow());
    specular *= occlusion(input, ray.clone(), 100.0); // TODO: Review.

    ambient + diffuse + specular
}
