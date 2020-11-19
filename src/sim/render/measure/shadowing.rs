//! Shadowing calculation.

use crate::{
    geom::Ray,
    math::{rand_circle_point, Dir3},
    sim::render::{occlusion, Input},
};
// use std::f64::consts::PI;

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

    let solar = if let Some((samples, rad)) = *input.shader.soft_shadow_samples() {
        // let offset = rng.gen_range(0.0, 2.0 * PI); TODO
        let offset = 0.0;
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = rand_circle_point(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * rad, theta + offset);
            total += occlusion(input, soft_ray, input.shader.occ_dist()[1]);
        }
        total / samples as f64
    } else {
        occlusion(input, light_ray, input.shader.occ_dist()[1])
    };

    solar
}
