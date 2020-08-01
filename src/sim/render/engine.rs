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

    // Event loop.
    loop {
        if let Some(hit) = input.tree.observe(trace.ray().clone(), bump_dist, 1_000.0) {
            trace.travel(hit.dist());
            let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
            col += colour(&mut rng, input, shader, &trace, &hit, &sun_dir) * trace.weight() as f32;
            break;
        } else {
            col += sky_col(shader, trace.ray(), &input.cols.map()["sky"]);
            break;
        }
    }

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
    let shadow = illumination::shadow(input, shader, trace.ray(), hit, rng);

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
    ray: &Ray,
    grad: &palette::Gradient<palette::LinSrgba>,
) -> palette::LinSrgba {
    let u = (ray.dir().dot(shader.cam().up()) + 1.0) * 0.5;
    let v = (ray.dir().dot(shader.cam().right()) + 1.0) * 0.5;

    let x = (shader.sky().noise().sample(u, v) + 1.0) * 0.5;

    let col = grad.get(x as f32);

    palette::Gradient::new(vec![palette::LinSrgba::default(), col])
        .get(shader.sky().brightness() as f32)
}
