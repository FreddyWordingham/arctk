//! Lighting calculation.

use crate::{
    geom::Ray,
    math::{rand_circle_point, rand_hemisphere_point, Dir3},
    phys::Crossing,
    sim::render::{Attribute, Input},
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Calculate the lighting factor.
/// Zero indicates darkness.
/// Unity indicates fully illuminated.
#[inline]
#[must_use]
pub fn light(input: &Input, ray: &Ray, norm: &Dir3) -> f64 {
    let light_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let view_dir = Dir3::new_normalize(input.cam.pos() - ray.pos());
    let ref_dir = Crossing::calc_ref_dir(ray.dir(), norm);

    let [ambient, mut diffuse, mut specular] = input.shader.light();
    diffuse *= norm.dot(&light_dir);
    specular *= view_dir
        .dot(&ref_dir)
        .max(0.0)
        .powi(input.shader.spec_pow());

    ambient + diffuse + specular
}

/// Calculate the shadowing factor.
/// Zero completely enshrouded.
/// Unity no shadows.
#[inline]
#[must_use]
pub fn shadow(input: &Input, rng: &mut ThreadRng, ray: &Ray, norm: &Dir3) -> f64 {
    let bump_dist = input.sett.bump_dist();

    let sun_dir = Dir3::new_normalize(input.shader.sun_pos() - ray.pos());
    let mut light_ray = Ray::new(*ray.pos(), *norm);
    light_ray.travel(bump_dist);
    *light_ray.dir_mut() = sun_dir;

    let solar = if let Some((samples, rad)) = input.shader.soft_shadow_samples() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        for n in 0..samples {
            let (r, theta) = rand_circle_point(n, samples);
            let mut soft_ray = light_ray.clone();
            soft_ray.rotate(r * rad, theta + offset);
            total += occlusion(input, soft_ray, input.shader.occ_dist()[1]);
        }
        total / f64::from(samples)
    } else {
        occlusion(input, light_ray, input.shader.occ_dist()[1])
    };

    if let Some((samples, power)) = input.shader.ambient_shadow_samples() {
        let offset = rng.gen_range(0.0, 2.0 * PI);
        let mut total = 0.0;
        let mut norm_ray = Ray::new(*ray.pos(), *norm);
        norm_ray.travel(bump_dist);
        for n in 0..samples {
            let (phi, theta) = rand_hemisphere_point(n, samples);
            let mut ambient_ray = norm_ray.clone();
            ambient_ray.rotate(phi, theta + offset);
            total += occlusion(input, ambient_ray, input.shader.occ_dist()[1]);
        }
        let ambient = (total / f64::from(samples)).powi(power);

        return ambient.mul_add(input.shader.shadow()[0], solar * input.shader.shadow()[1]);
    };

    solar
}

/// Calculate the occlusion experienced over a distance along ray.
/// Zero indicates full occlusion.
/// Unity indicates full view.
#[inline]
#[must_use]
pub fn occlusion(input: &Input, mut ray: Ray, mut dist: f64) -> f64 {
    debug_assert!(dist > 0.0);

    let bump_dist = input.sett.bump_dist();
    let loop_limit = input.sett.loop_limit();
    let min_weight = input.sett.min_weight();

    let mut vis = 1.0;
    let mut num_loops = 0;
    while let Some(hit) = input.tree.scan(ray.clone(), bump_dist, dist) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating shadower: loop limit reached.");
            return 0.0;
        }
        num_loops += 1;

        // Check if we've flown far enough.
        dist -= hit.dist();
        if dist < 0.0 {
            return vis;
        }

        // Check if it's still worth going.
        if vis < min_weight {
            return 0.0;
        }

        // Handle collision.
        match *hit.tag() {
            Attribute::Opaque(..) => {
                return vis / dist.mul_add(input.shader.fall_off(), 1.0);
            }
            Attribute::Mirror(.., abs_frac) => {
                ray.travel(dist);
                vis *= 1.0 - abs_frac;
                *ray.dir_mut() = Crossing::calc_ref_dir(ray.dir(), hit.side().norm());
                ray.travel(bump_dist);
            }
            Attribute::Transparent(.., abs_frac) => {
                ray.travel(dist + bump_dist);
                vis *= 1.0 - abs_frac;
            }
            Attribute::Refractive(.., abs_frac, [_inside, _outside]) => {
                ray.travel(dist + bump_dist);
                vis *= 1.0 - abs_frac;
            }
            Attribute::Luminous(.., bright_mult) => {
                return (vis * bright_mult) / dist.mul_add(input.shader.fall_off(), 1.0);
            }
        }
    }

    vis
}
