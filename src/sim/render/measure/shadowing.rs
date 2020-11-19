//! Shadowing calculation.

use crate::{
    geom::Ray,
    math::Dir3,
    sim::render::{occlusion, Input},
};

/// Calculate the shadowing factor.
/// Zero completely enshrouded.
/// Unity no shadows.
#[inline]
#[must_use]
pub fn shadowing(input: &Input, ray: &Ray, norm: &Dir3) -> f64 {
    let bump_dist = input.sett.bump_dist();

    let sun_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *norm);
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;

    occlusion(input, light_ray, input.shader.occ_dist()[1])
}
