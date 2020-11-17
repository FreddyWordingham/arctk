//! Ray observation engine function.

use crate::sim::cartographer::{Input, Output};
use rand::rngs::ThreadRng;

/// Determine what a single ray observes.
#[allow(clippy::expect_used)]
#[inline]
pub fn basic(input: &Input, mut rng: &mut ThreadRng, index: [usize; 3], mut data: &mut Output) {}
