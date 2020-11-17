//! Ray observation engine function.

use crate::sim::cartographer::{Input, Output};
use rand::rngs::ThreadRng;

/// Determine what a single ray observes.
#[allow(clippy::expect_used)]
#[inline]
pub fn basic(input: &Input, mut _rng: &mut ThreadRng, _index: [usize; 3], mut _data: &mut Output) {
    let bump_dist = input.sett.bump_dist();

    let super_samples = input.sett.super_sampling().num_samples();
    let weight = 1.0 / super_samples as f64;

    let num_casts = input.sett.caster().num_casts();

    for n in 0..super_samples {}
}
