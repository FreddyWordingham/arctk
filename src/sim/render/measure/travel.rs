//! Tracer movement.

use crate::sim::render::{Output, Tracer};

/// Travel the tracer forward and record the flight.
#[inline]
pub fn travel(trace: &mut Tracer, data: &mut Output, pixel: [usize; 2], dist: f64) {
    debug_assert!(dist > 0.0);

    trace.ray_mut().travel(dist);
    data.dist[pixel] += dist;
}
