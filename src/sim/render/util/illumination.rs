//! Illumination functions.

use crate::{
    golden,
    render::{Attributes, Input, Shader, Tracer},
    Crossing, Dir3, Hit, Ray,
};
use rand::{rngs::ThreadRng, Rng};
use std::f64::consts::PI;

/// Maximum distance tested for ray visibility [m].
const MAX_VISIBILITY_DIST: f64 = 10.0;

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
pub fn shadow(input: &Input, shader: &Shader, ray: &Ray, hit: &Hit, rng: &mut ThreadRng) -> f64 {
    let bump_dist = input.sett.bump_dist();

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
            total += visibility(input, Tracer::new(soft_ray, -1), 1.0);
        }
        total / f64::from(samples)
    } else {
        visibility(input, Tracer::new(light_ray, -1), 1.0)
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
            total += visibility(input, Tracer::new(ambient_ray, -1), 1.0);
        }
        let ambient = (total / f64::from(samples)).powi(shader.shadow().ao_pow());

        return ambient.mul_add(*shader.shadow().ambient(), solar * shader.shadow().direct());
    };

    solar
}

/// Calculate the visibility of a given ray.
#[inline]
#[must_use]
pub fn visibility(input: &Input, mut trace: Tracer, mut vis: f64) -> f64 {
    debug_assert!(vis > 0.0);
    debug_assert!(vis <= 1.0);

    let bump_dist = input.sett.bump_dist();

    while let Some(hit) = input.tree.observe(
        trace.ray().clone(),
        bump_dist,
        MAX_VISIBILITY_DIST - trace.dist_travelled(),
    ) {
        let group = hit.group();
        if let Some(attr) = input.attrs.map().get(group) {
            match attr {
                Attributes::Transparent { abs } => {
                    vis *= 1.0 - *abs;
                    trace.travel(hit.dist() + bump_dist);
                }
                Attributes::Mirror { abs } => {
                    trace.travel(hit.dist());
                    vis *= 1.0 - *abs;
                    trace.set_dir(Crossing::calc_ref_dir(trace.dir(), hit.side().norm()));
                    trace.travel(bump_dist);
                }
                Attributes::Luminous => {
                    return vis;
                }
            }
        } else {
            return 0.0;
        }

        if trace.dist_travelled() >= MAX_VISIBILITY_DIST {
            break;
        }
    }

    vis
}
