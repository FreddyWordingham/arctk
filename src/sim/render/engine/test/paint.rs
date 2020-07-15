//! Image painter function.

use crate::{
    render::{Input, Scene},
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
    mut _rng: &mut ThreadRng,
    _input: &Input,
    _scene: &Scene,
    mut _ray: Ray,
    mut _weight: f64,
) -> LinSrgba {
    LinSrgba::default()
}
