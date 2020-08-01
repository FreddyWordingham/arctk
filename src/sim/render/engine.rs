//! Image painter function.

use super::{illumination, Collision};
use crate::{
    render::{Attributes, Input, Shader, Tracer},
    Crossing, Dir3, Hit, Ray, Trace,
};
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;

/// Rendering engine painter function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn paint(
    mut rng: &mut ThreadRng,
    input: &Input,
    shader: &Shader,
    mut trace: Tracer,
) -> LinSrgba {
    debug_assert!(trace.weight() > 0.0);
    debug_assert!(trace.weight() <= 1.0);

    // Convenience.
    let bump_dist = input.sett.bump_dist();
    let sun_pos = shader.sky().sun_pos();

    // Tracked items.
    let mut col = LinSrgba::default();

    col
}

/// Perform a colouring.
#[inline]
fn colour(
    rng: &mut ThreadRng,
    input: &Input,
    shader: &Shader,
    trace: &Tracer,
    hit: &Hit,
    sun_dir: &Dir3,
) -> LinSrgba {
    let light = illumination::light(shader, trace.ray(), hit);
    let shadow = illumination::shadow(input, shader, trace.ray(), hit, input.sett.bump_dist(), rng);

    let x = hit.side().norm().dot(sun_dir).abs();

    let base_col = input.cols.map()[hit.group()].get(x as f32);
    let grad = Gradient::new(vec![LinSrgba::default(), base_col]);

    grad.get((light * shadow) as f32)
}

/// Determine the sky colour.
#[inline]
#[must_use]
fn sky_col(
    shader: &Shader,
    trace: &Tracer,
    grad: &palette::Gradient<palette::LinSrgba>,
) -> palette::LinSrgba {
    let u = (trace.dir().dot(shader.cam().up()) + 1.0) * 0.5;
    let v = (trace.dir().dot(shader.cam().right()) + 1.0) * 0.5;

    let x = (shader.sky().noise().sample(u, v) + 1.0) * 0.5;

    let col = grad.get(x as f32);

    palette::Gradient::new(vec![palette::LinSrgba::default(), col])
        .get(shader.sky().brightness() as f32)
}
