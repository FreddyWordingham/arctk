//! Illumination functions.

use crate::{
    golden,
    render::{Input, Shader},
    Crossing, Dir3, Hit, Ray,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Maximum distance tested for ray visibility [m].
const MAX_VISIBILITY_DIST: f64 = 1.0e9;

/// Calculate the lighting factor.
#[inline]
#[must_use]
pub fn light(shader: &Shader, ray: &Ray, hit: &Hit) -> f64 {
    let light_dir = Dir3::new_normalize(shader.sky().sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(shader.cam().focus().orient().pos() - ray.pos());
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

/// Calculate the shadowing factor.
#[inline]
#[must_use]
pub fn shadow(
    input: &Input,
    shader: &Shader,
    ray: &Ray,
    hit: &Hit,
    bump_dist: f64,
    rng: &mut ThreadRng,
) -> f64 {
    debug_assert!(bump_dist > 0.0);

    let sun_dir = Dir3::new_normalize(shader.sky().sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *hit.side().norm());
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;

    let solar = if let Some(samples) = shader.samples().soft_shadows() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = golden::circle(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * shader.sky().sun_rad(), theta + offset);
            total += visibility(input, soft_ray, bump_dist, 1.0, 0.0, 1000.0);
        }
        total / f64::from(samples)
    } else {
        visibility(input, light_ray, bump_dist, 1.0, 0.0, 1000.0)
    };

    if let Some(samples) = shader.samples().ambient_occlusion() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *hit.side().norm());
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = golden::hemisphere(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += visibility(input, ambient_ray, bump_dist, 1.0, 0.0, 10.0);
        }
        let ambient = (total / f64::from(samples)).powi(shader.shadow().ao_pow());

        return ambient.mul_add(*shader.shadow().ambient(), solar * shader.shadow().direct());
    };

    solar
}

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
pub fn visibility(
    input: &Input,
    mut ray: Ray,
    bump_dist: f64,
    mut vis: f64,
    mut dist: f64,
    max_dist: f64,
) -> f64 {
    debug_assert!(vis > 0.0);
    debug_assert!(vis <= 1.0);
    debug_assert!(bump_dist > 0.0);

    vis
}
