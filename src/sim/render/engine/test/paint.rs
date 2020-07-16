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
) -> (LinSrgba, f64) {
    debug_assert!(weight > 0.0);
    debug_assert!(weight <= 1.0);

    let bump_dist = input.sett.bump_dist();
    let mut col = LinSrgba::default();
    let mut total_dist = 0.0;

    // Move rays into the grid.
    if !input.grid.boundary().contains(ray.pos()) {
        if let Some(dist) = input.grid.boundary().dist(&ray) {
            total_dist += dist + bump_dist;
            ray.travel(dist + bump_dist)
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
                    total_dist += dist + bump_dist;
                    ray.travel(dist + bump_dist);
                    // col += sky_col(scene, &ray, &input.cols.map()["sky"]) * weight as f32;
                    break;
                }
                Event::Surface(hit) => {
                    let group = hit.group();
                    if let Some(attr) = input.attrs.map().get(group) {
                        match attr {
                            Attributes::Transparent { abs } => {
                                ray.travel(hit.dist());
                                col += colour(&mut rng, input, scene, &ray, &hit)
                                    * *abs as f32
                                    * weight as f32;
                                weight *= 1.0 - *abs;
                                ray.travel(bump_dist);
                                total_dist += hit.dist() + bump_dist;
                            }
                            Attributes::Mirror { abs } => {
                                ray.travel(hit.dist());
                                col += colour(&mut rng, input, scene, &ray, &hit)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - *abs;
                                *ray.dir_mut() =
                                    Crossing::calc_ref_dir(ray.dir(), hit.side().norm());
                                ray.travel(bump_dist);
                                total_dist += hit.dist() + bump_dist;
                            }
                            Attributes::Refractive {
                                abs,
                                inside,
                                outside,
                            } => {
                                ray.travel(hit.dist());
                                col += colour(&mut rng, input, scene, &ray, &hit)
                                    * (*abs * weight) as f32;
                                weight *= 1.0 - abs;

                                let (n_curr, n_next) = if hit.side().is_inside() {
                                    (*inside, *outside)
                                } else {
                                    (*outside, *inside)
                                };
                                let crossing =
                                    Crossing::new(ray.dir(), hit.side().norm(), n_curr, n_next);

                                let trans_prob = crossing.trans_prob();
                                if let Some(trans_dir) = crossing.trans_dir() {
                                    let mut trans_ray = ray.clone();
                                    *trans_ray.dir_mut() = *trans_dir;
                                    trans_ray.travel(bump_dist);
                                    let (c, d) =
                                        paint(rng, input, scene, trans_ray, weight * trans_prob);
                                    col += c * weight as f32;
                                    total_dist += d;
                                }

                                weight *= crossing.ref_prob();
                                *ray.dir_mut() = *crossing.ref_dir();
                                ray.travel(bump_dist);
                                total_dist += hit.dist() + bump_dist;
                            }
                        }
                    } else {
                        ray.travel(hit.dist());
                        col += colour(&mut rng, input, scene, &ray, &hit) * weight as f32;
                        total_dist += hit.dist();
                        break;
                    }
                }
            }

            if weight < input.sett.min_weight() {
                break;
            }
        }
    } else {
        // col += sky_col(scene, &ray, &input.cols.map()["sky"]);
    }

    (col, total_dist)
}

/// Perform a colouring.
#[inline]
fn colour(rng: &mut ThreadRng, input: &Input, scene: &Scene, ray: &Ray, hit: &Hit) -> LinSrgba {
    let light = illumination::light(scene, ray, hit);
    let shadow = illumination::shadow(input, scene, ray, hit, input.sett.bump_dist(), rng);

    let sun_dir = Dir3::new_normalize(ray.pos() - scene.lighting().sky().sun_pos());
    let base_col = input.cols.map()[hit.group()].get(hit.side().norm().dot(&sun_dir).abs() as f32);
    let grad = palette::Gradient::new(vec![palette::LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}
