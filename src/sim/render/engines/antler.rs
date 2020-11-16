//! Pixel-sampling engine function.

use crate::sim::render::{Input, Output, Tracer};
use rand::rngs::ThreadRng;

/// Calculate the colour of a single tracer.
#[allow(clippy::expect_used)]
#[inline]
pub fn antler(
    _input: &Input,
    mut _rng: &mut ThreadRng,
    mut _trace: Tracer,
    mut _data: &mut Output,
) {
}
