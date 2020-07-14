//! Rendering engine: Test.

pub mod event;

pub use self::event::*;

use crate::{
    render::{Input, Scene},
    Ray,
};
use palette::LinSrgba;
use rand::rngs::ThreadRng;

/// Test rendering engine function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn engine(
    mut rng: &mut ThreadRng,
    input: &Input,
    scene: &Scene,
    mut ray: Ray,
    mut weight: f64,
) -> LinSrgba {
    LinSrgba::default()
}
