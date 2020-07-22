//! Image painter function.

use super::{illumination, Event};
use crate::{
    render::{Attributes, Input, Scene},
    Crossing, Dir3, Hit, Ray, Trace,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Test rendering engine painter function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn paint(
    mut rng: &mut ThreadRng,
    input: &Input,
    scene: &Scene,
    mut ray: Ray,
    mut weight: f64,
) -> (LinSrgba, i32, i32) {
    debug_assert!(weight > 0.0);
    debug_assert!(weight <= 1.0);

    let bump_dist = input.sett.bump_dist();
    let sun_pos = scene.lighting().sky().sun_pos();

    // Tracked items.
    let mut col = LinSrgba::default();
    let mut first_hit = None;
    let mut last_hit = None;

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            ray.travel(dist + bump_dist);
        }
    }

    if let Some((_index, voxel)) = input.grid.gen_index_voxel(ray.pos()) {
        loop {
            // Determine possible event distances.
            let voxel_dist = voxel
                .dist(&ray)
                .expect("Could not determine voxel distance.");
            let surf_hit = input.tree.observe(ray.clone(), bump_dist, voxel_dist);

            // Handle event.
            match Event::new(voxel_dist, surf_hit) {
                Event::Voxel(dist) => {
                    ray.travel(dist);
                    col += sky_col(scene, &ray, &input.cols.map()["sky"]) * weight as f32;
                    break;
                }
                Event::Surface(hit) => {
                    let group = hit.group();

                    if first_hit.is_none() {
                        first_hit =
                            Some(input.cols.index_of(group).expect("Missing hit entry.") as i32);
                        last_hit = first_hit;
                    } else {
                        last_hit =
                            Some(input.cols.index_of(group).expect("Missing hit entry.") as i32);
                    }

                    if let Some(attr) = input.attrs.map().get(group) {
                        match attr {
                            Attributes::Transparent { abs } => {
                                ray.travel(hit.dist());
                                let sun_dir = Dir3::new_normalize(ray.pos() - sun_pos);
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * *abs as f32
                                    * weight as f32;
                                weight *= 1.0 - *abs;
                                ray.travel(bump_dist);
                            }
                            Attributes::Mirror { abs } => {
                                ray.travel(hit.dist());
                                let sun_dir = Dir3::new_normalize(ray.pos() - sun_pos);
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - *abs;
                                *ray.dir_mut() =
                                    Crossing::calc_ref_dir(ray.dir(), hit.side().norm());
                                ray.travel(bump_dist);
                            }
                            Attributes::Refractive {
                                abs,
                                inside,
                                outside,
                            } => {
                                ray.travel(hit.dist());
                                let sun_dir = Dir3::new_normalize(ray.pos() - sun_pos);
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - abs;

                                let (n_curr, n_next) = if hit.side().is_inside() {
                                    (*inside, *outside)
                                } else {
                                    (*outside, *inside)
                                };
                                let crossing =
                                    Crossing::new(ray.dir(), hit.side().norm(), n_curr, n_next);

                                // Reflection ray.
                                let ref_prob = crossing.ref_prob();
                                if ref_prob >= input.sett.min_weight() {
                                    let mut ref_ray = ray.clone();
                                    *ref_ray.dir_mut() = *crossing.ref_dir();
                                    ref_ray.travel(bump_dist);
                                    let (c, _fhi, lhi) =
                                        paint(rng, input, scene, ref_ray, weight * ref_prob);

                                    col += c * (weight * ref_prob) as f32;
                                    last_hit = Some(lhi);
                                }

                                // Transmission ray.
                                let trans_prob = crossing.trans_prob();
                                if trans_prob >= input.sett.min_weight() {
                                    if let Some(trans_dir) = crossing.trans_dir() {
                                        let mut trans_ray = ray.clone();
                                        *trans_ray.dir_mut() = *trans_dir;
                                        trans_ray.travel(bump_dist);
                                        let (c, _fhi, lhi) = paint(
                                            rng,
                                            input,
                                            scene,
                                            trans_ray,
                                            weight * trans_prob,
                                        );

                                        col += c * (weight * trans_prob) as f32;
                                        last_hit = Some(lhi);
                                    }
                                }

                                break;
                            }
                            Attributes::Luminous => {
                                ray.travel(hit.dist());
                                let sun_dir = Dir3::new_normalize(ray.pos() - sun_pos);
                                col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir)
                                    * weight as f32;
                                break;
                            }
                        }
                    } else {
                        ray.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(ray.pos() - sun_pos);
                        col += colour(&mut rng, input, scene, &ray, &hit, &sun_dir) * weight as f32;
                        break;
                    }
                }
            }

            if weight < input.sett.min_weight() {
                break;
            }
        }
    } else {
        col += sky_col(scene, &ray, &input.cols.map()["sky"]);
        println!("Sky");
    }

    (col, first_hit.unwrap_or(-1), last_hit.unwrap_or(-1))
}

/// Perform a colouring.
#[inline]
fn colour(
    rng: &mut ThreadRng,
    input: &Input,
    scene: &Scene,
    ray: &Ray,
    hit: &Hit,
    sun_dir: &Dir3,
) -> LinSrgba {
    let light = (illumination::light(scene, ray, hit) + 0.5).min(1.0);
    let shadow = illumination::shadow(input, scene, ray, hit, input.sett.bump_dist(), rng);

    let x = hit.side().norm().dot(sun_dir).abs();
    // if x >= 0.75 {
    //     // Cel shading.
    //     x = 1.0;
    // } else if x > 0.5 {
    //     x = 0.6;
    // } else {
    //     x = 0.3;
    // }

    let base_col = input.cols.map()[hit.group()].get(x as f32);
    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(
    scene: &Scene,
    ray: &Ray,
    grad: &palette::Gradient<palette::LinSrgba>,
) -> palette::LinSrgba {
    let u = (ray.dir().dot(scene.cam().focus().orient().up()) + 1.0) * 0.5;
    let v = (ray.dir().dot(scene.cam().focus().orient().right()) + 1.0) * 0.5;

    let x = (scene.lighting().sky().noise().sample(u, v) + 1.0) * 0.5;

    let col = grad.get(x as f32);

    palette::Gradient::new(vec![palette::LinSrgba::default(), col])
        .get(scene.lighting().sky().brightness() as f32)
}
