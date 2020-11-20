//! Pixel-sampling engine function.

use crate::{
    geom::Ray,
    img::{Colour, Gradient},
    math::Dir3,
    phys::Crossing,
    sim::render::{lighting, shadowing, Attribute, Input, Output, Tracer},
};
use rand::rngs::ThreadRng;
use std::time::Instant;

/// Calculate the colour of a single tracer.
#[allow(clippy::expect_used)]
#[inline]
pub fn antler(
    input: &Input,
    rng: &mut ThreadRng,
    mut trace: Tracer,
    data: &mut Output,
    pixel: [usize; 2],
) {
    // Watch time.
    let start_time = Instant::now();

    // Common constants.
    let bump_dist = input.sett.bump_dist();
    let loop_limit = input.sett.loop_limit();
    let min_weight = input.sett.min_weight();

    // First hit.
    if let Some(hit) = input.tree.scan(trace.ray().clone(), bump_dist, 1000.0) {
        // Handle collision.
        let norm = hit.side().norm();
        match *hit.tag() {
            Attribute::Opaque(grad) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, 1.0);
            }
            Attribute::Mirror(grad, abs_frac) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, abs_frac);
                *trace.ray_mut().dir_mut() = Crossing::calc_ref_dir(trace.ray().dir(), norm);
                trace.ray_mut().travel(bump_dist);
            }
            Attribute::Transparent(grad, abs_frac) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, abs_frac);
                trace.ray_mut().travel(bump_dist);
            }
            Attribute::Refractive(grad, abs_frac, [inside, outside]) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, abs_frac);

                let [curr, next] = if hit.side().is_inside() {
                    [inside, outside]
                } else {
                    [outside, inside]
                };
                let crossing = Crossing::new(trace.ray().dir(), norm, curr, next);

                // Transmission ray.
                if let Some(trans_dir) = *crossing.trans_dir() {
                    let mut trans_trace = trace.clone();
                    *trans_trace.ray_mut().dir_mut() = trans_dir;
                    trans_trace.ray_mut().travel(bump_dist);

                    *trans_trace.weight_mut() *= crossing.trans_prob();
                    antler(input, rng, trans_trace, data, pixel);
                }

                // Continuing reflection ray.
                *trace.weight_mut() *= crossing.ref_prob();
                *trace.ray_mut().dir_mut() = *crossing.ref_dir();
                trace.ray_mut().travel(bump_dist);
            }
            Attribute::Luminous(grad, bright_mult) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, bright_mult);
            }
        }
    } else {
        sky_colour(input, trace.ray(), data, pixel);
    }

    // Main event loop.
    let mut num_loops = 0;
    while let Some(hit) = input.tree.scan(trace.ray().clone(), bump_dist, 1000.0) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating tracer: loop limit reached.");
            break;
        }
        num_loops += 1;

        // Weight pruning.
        if trace.weight() < min_weight {
            break;
        }

        // Handle collision.
        let norm = hit.side().norm();
        match *hit.tag() {
            Attribute::Opaque(grad) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, 1.0);
                break;
            }
            Attribute::Mirror(grad, abs_frac) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, abs_frac);
                *trace.ray_mut().dir_mut() = Crossing::calc_ref_dir(trace.ray().dir(), norm);
                trace.ray_mut().travel(bump_dist);
            }
            Attribute::Transparent(grad, abs_frac) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, abs_frac);
                trace.ray_mut().travel(bump_dist);
            }
            Attribute::Refractive(grad, abs_frac, [inside, outside]) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, abs_frac);

                let [curr, next] = if hit.side().is_inside() {
                    [inside, outside]
                } else {
                    [outside, inside]
                };
                let crossing = Crossing::new(trace.ray().dir(), norm, curr, next);

                // Transmission ray.
                if let Some(trans_dir) = *crossing.trans_dir() {
                    let mut trans_trace = trace.clone();
                    *trans_trace.ray_mut().dir_mut() = trans_dir;
                    trans_trace.ray_mut().travel(bump_dist);

                    *trans_trace.weight_mut() *= crossing.trans_prob();
                    antler(input, rng, trans_trace, data, pixel);
                    break;
                }

                // Continuing reflection ray.
                *trace.weight_mut() *= crossing.ref_prob();
                *trace.ray_mut().dir_mut() = *crossing.ref_dir();
                trace.ray_mut().travel(bump_dist);
            }
            Attribute::Luminous(grad, bright_mult) => {
                trace.ray_mut().travel(hit.dist());
                colour(input, rng, &mut trace, norm, grad, data, pixel, bright_mult);
                break;
            }
        }
    }

    // Record time.
    data.time[pixel] += start_time.elapsed().as_micros() as f64;
}

/// Determine the colour of the sky.
/// Record the data.
#[inline]
fn sky_colour(input: &Input, ray: &Ray, data: &mut Output, pixel: [usize; 2]) {
    // Colour calculation.
    let u = 1.0 - ray.dir().z.abs();
    let col = input.sett.sky_grad().get(u as f32);

    // Data recording.
    data.shadow[pixel] += 1.0;
    data.light[pixel] += 1.0;

    // Colouring.
    data.colour.pixels_mut()[pixel] += col;
}

/// Determine the colour of a ray-surface collision.
/// Record the data.
#[inline]
fn colour(
    input: &Input,
    rng: &mut ThreadRng,
    trace: &mut Tracer,
    norm: &Dir3,
    grad: &Gradient,
    data: &mut Output,
    pixel: [usize; 2],
    abs_frac: f64,
) {
    debug_assert!(abs_frac > 0.0);
    debug_assert!(abs_frac <= 1.0);

    // Colour calculation.
    let shadow = shadowing(input, rng, trace.ray(), norm);
    let light = lighting(input, trace.ray(), norm);
    let base_col = grad.get(light as f32);
    // let col = Gradient::new(vec![Colour::new(0.0, 0.0, 0.0, 0.0), base_col]).get(shadow as f32);
    // let col = Gradient::new(vec![Colour::new(0.0, 0.0, 0.0, 1.0), base_col]).get(shadow as f32);
    let col = Gradient::new(vec![Colour::default(), base_col]).get(shadow as f32);

    // Weighting.
    let weight = trace.weight() * abs_frac;
    *trace.weight_mut() *= 1.0 - abs_frac;

    // Data recording.
    data.shadow[pixel] += shadow * weight;
    data.light[pixel] += light * weight;
    data.final_norm[pixel] += weight * norm.as_ref();

    // Colouring.
    data.colour.pixels_mut()[pixel] += col * weight as f32;
}
