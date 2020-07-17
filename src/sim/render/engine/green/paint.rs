//! Image painter function.

// use super::{illumination, Event};
use crate::{
    render::{
        // Attributes,
        Input,
        Scene,
    },
    // Crossing, Dir3, Hit, Ray, Trace,
    Ray,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Test rendering engine painter function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn paint(
    _rng: &mut ThreadRng,
    input: &Input,
    _scene: &Scene,
    _ray: Ray,
    weight: f64,
) -> LinSrgba {
    debug_assert!(weight > 0.0);
    debug_assert!(weight <= 1.0);

    let _bump_dist = input.sett.bump_dist();
    let col = LinSrgba::default();

    col
}
