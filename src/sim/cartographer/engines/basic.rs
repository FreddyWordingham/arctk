//! Ray observation engine function.

use crate::sim::cartographer::{Input, Output};
use rand::rngs::ThreadRng;

/// Determine what a single ray observes.
#[allow(clippy::expect_used)]
#[inline]
pub fn basic(_input: &Input, mut _rng: &mut ThreadRng, _index: [usize; 3], mut _data: &mut Output) {
}
