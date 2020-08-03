//! Engine sampling function.

use crate::{
    render::{illumination, Attributes, Sample, Scene, Shader, Tracer},
    Crossing, Dir3, Hit, Ray,
};
use palette::{Gradient, LinSrgba};
use rand::rngs::ThreadRng;

/// Rendering engine sampling function.
#[allow(clippy::never_loop)]
#[allow(clippy::option_expect_used)]
#[allow(clippy::single_match_else)]
#[allow(clippy::too_many_lines)]
#[inline]
#[must_use]
pub fn paint(mut rng: &mut ThreadRng, scene: &Scene, shader: &Shader, mut trace: Tracer) -> Sample {
    debug_assert!(trace.weight() > 0.0);
    debug_assert!(trace.weight() <= 1.0);

    // Convenience.
    let bump_dist = scene.sett.bump_dist();
    let sun_pos = shader.sky().sun_pos();

    // Tracked items.
    let mut col = LinSrgba::default();

    // Event loop.
    loop {
        if trace.weight() <= scene.sett.min_weight() {
            break;
        }

        if let Some(hit) = scene.tree.observe(trace.ray().clone(), bump_dist, 1_000.0) {
            if let Some(attr) = scene.attrs.map().get(hit.group()) {
                match attr {
                    Attributes::Transparent { abs } => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * *abs as f32
                            * trace.weight() as f32;
                        *trace.weight_mut() *= 1.0 - *abs;
                        trace.travel(bump_dist);
                    }
                    Attributes::Mirror { abs } => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * (*abs * trace.weight()) as f32;
                        *trace.weight_mut() *= 1.0 - *abs;
                        trace.set_dir(Crossing::calc_ref_dir(trace.dir(), hit.side().norm()));
                        trace.travel(bump_dist);
                    }
                    Attributes::Luminous => {
                        trace.travel(hit.dist());
                        let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                        col += colour(&mut rng, scene, shader, &trace, &hit, &sun_dir)
                            * trace.weight() as f32;
                        break;
                    }
                }
            } else {
                trace.travel(hit.dist());
                let sun_dir = Dir3::new_normalize(trace.pos() - sun_pos);
                col +=
                    colour(&mut rng, scene, shader, &trace, &hit, &sun_dir) * trace.weight() as f32;
                break;
            }
        } else {
            col += sky_col(shader, trace.ray(), &scene.cols.map()["sky"]);
            break;
        }
    }

    Sample::new(col)
}

/// Perform a colouring.
#[inline]
fn colour(
    rng: &mut ThreadRng,
    scene: &Scene,
    shader: &Shader,
    trace: &Tracer,
    hit: &Hit,
    sun_dir: &Dir3,
) -> LinSrgba {
    let light = illumination::light(shader, trace.ray(), hit);
    let shadow = illumination::shadow(scene, shader, trace.ray(), hit, rng);

    let x = hit.side().norm().dot(sun_dir).abs();

    let base_col = scene.cols.map()[hit.group()].get(x as f32);
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
