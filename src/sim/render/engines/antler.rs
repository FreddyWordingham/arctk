//! Pixel-sampling engine function.

use crate::sim::render::{Attribute, Input, Output, Tracer};
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
        match *hit.tag() {
            Attribute::Opaque(ref grad) => {
                travel(&mut trace, &mut data, pixel, hit.dist());
                data.colour.pixels_mut()[pixel] += grad.get(1.0) * *trace.weight() as f32;
                break;
            }
            Attribute::Mirror(..) => {}
        }

        travel(&mut trace, &mut data, pixel, bump_dist);
    }

    // Record time.
    data.time[pixel] += start_time.elapsed().as_micros() as f64;
}

/// Travel the tracer forward and record the flight.
#[inline]
pub fn travel(trace: &mut Tracer, data: &mut Output, pixel: [usize; 2], dist: f64) {
    debug_assert!(dist > 0.0);

    trace.ray_mut().travel(dist);
    data.dist[pixel] += dist;
}
