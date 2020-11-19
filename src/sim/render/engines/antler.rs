//! Pixel-sampling engine function.

use crate::{
    img::{Colour, Gradient},
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
                travel(&mut trace, &mut data, pixel, hit.dist());
                let shadow = shadowing(input, trace.ray(), norm);
                let light = lighting(input, trace.ray(), norm);
                data.light[pixel] += light;
                data.shadow[pixel] += shadow;
                data.final_norm[pixel] += hit.side().norm().as_ref();
                data.block_colour.pixels_mut()[pixel] += Gradient::new(vec![
                    Colour::new(0.0, 0.0, 0.0, 0.0),
                    grad.get(light as f32),
                ])
                .get(shadow as f32)
                    * *trace.weight() as f32;
                break;
            }
            Attribute::Mirror(ref_frac) => {
                travel(&mut trace, &mut data, pixel, hit.dist());
                *trace.weight_mut() *= ref_frac;
                *trace.ray_mut().dir_mut() = Crossing::calc_ref_dir(trace.ray().dir(), norm);
                travel(&mut trace, &mut data, pixel, bump_dist);
            }
        }
    }

    // Record time.
    data.time[pixel] += start_time.elapsed().as_micros() as f64;
}
