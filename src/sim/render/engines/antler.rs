//! Pixel-sampling engine function.

use crate::sim::render::{Input, Output, Tracer};
use rand::rngs::ThreadRng;

/// Calculate the colour of a single tracer.
#[allow(clippy::expect_used)]
#[inline]
pub fn antler(input: &Input, mut rng: &mut ThreadRng, mut trace: Tracer, mut data: &mut Output) {}
