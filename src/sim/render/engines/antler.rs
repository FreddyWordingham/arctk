//! Pixel-sampling engine function.

use crate::{
    img::{Colour, Gradient},
    math::Dir3,
    phys::Crossing,
    sim::render::{lighting, shadowing, travel, Attribute, Input, Output, Tracer},
};
use rand::rngs::ThreadRng;
use std::time::Instant;

/// Calculate the colour of a single tracer.
#[allow(clippy::expect_used)]
#[inline]
pub fn antler(
    input: &Input,
    mut _rng: &mut ThreadRng,
    mut trace: Tracer,
    mut data: &mut Output,
    pixel: [usize; 2],
) {
    // Watch time.
    let start_time = Instant::now();

    // Common constants.
    let bump_dist = input.sett.bump_dist();
    let loop_limit = input.sett.loop_limit();
    let _min_weight = input.sett.min_weight();

    // Main event loop.
    let mut num_loops = 0;
    while let Some(hit) = input.tree.scan(trace.ray().clone(), bump_dist, 1000.0) {
        // Loop limit check.
        if num_loops >= loop_limit {
            println!("[WARN] : Terminating tracer: loop limit reached.");
            break;
        }
        num_loops += 1;

        // Handle collision.
        let norm = hit.side().norm();
        match *hit.tag() {
            Attribute::Opaque(grad) => {
                travel(&mut trace, &mut data, pixel, hit.dist() - bump_dist);
                colour(input, &trace, norm, grad, data, pixel);
                break;
            }
            Attribute::Mirror(_grad, ref_frac) => {
                travel(&mut trace, &mut data, pixel, hit.dist() = bump_dist);
                *trace.weight_mut() *= ref_frac;
                colour(input, &trace, norm, grad, data, pixel);
                *trace.ray_mut().dir_mut() = Crossing::calc_ref_dir(trace.ray().dir(), norm);
                travel(&mut trace, &mut data, pixel, 2.0 * bump_dist);
            }
            Attribute::Transparent(_grad, trans_frac) => {
                travel(&mut trace, &mut data, pixel, hit.dist() - bump_dist);
                *trace.weight_mut() *= trans_frac;
                colour(input, &trace, norm, grad, data, pixel);
                travel(&mut trace, &mut data, pixel, 2.0 * bump_dist);
            }
        }
    }

    // Record time.
    data.time[pixel] += start_time.elapsed().as_micros() as f64;
}

/// Determine the colour of a ray-surface collision.
/// Record the data.
#[inline]
fn colour(
    input: &Input,
    trace: &Tracer,
    norm: &Dir3,
    grad: &Gradient,
    data: &mut Output,
    pixel: [usize; 2],
) {
    // Colour calculation.
    let shadow = shadowing(input, trace.ray(), norm);
    let light = lighting(input, trace.ray(), norm);
    let base_col = grad.get(light as f32);
    // let col = Gradient::new(vec![Colour::new(0.0, 0.0, 0.0, 0.0), base_col]).get(shadow as f32);
    // let col = Gradient::new(vec![Colour::new(0.0, 0.0, 0.0, 1.0), base_col]).get(shadow as f32);
    let col = Gradient::new(vec![Colour::default(), base_col]).get(shadow as f32);

    // Data recording.
    let weight = trace.weight();

    // Data recording.
    data.light[pixel] += light * weight;
    data.shadow[pixel] += shadow * weight;
    data.final_norm[pixel] += weight * norm.as_ref();

    // Colouring.
    data.colour.pixels_mut()[pixel] += col * weight as f32;
}
