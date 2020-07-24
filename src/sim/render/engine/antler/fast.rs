//! Fast-scheme rendering function.

use super::{paint, Output};
use crate::{
    render::{Input, Scene},
    Bar, Dir3, Error, Vec3,
};
use palette::{Gradient, LinSrgba};
use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::{
    f64::consts::PI,
    sync::{Arc, Mutex},
};

/// Render an image fast.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[allow(clippy::option_expect_used)]
#[allow(clippy::result_expect_used)]
#[inline]
pub fn run(input: &Input, scene: &Scene) -> Result<Output, Error> {
    let num_pixels = scene.cam().sensor().num_pixels();
    let bar = Bar::new("Rendering", num_pixels as u64);
    let bar = Arc::new(Mutex::new(bar));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut out: Vec<Output> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&bar), input, scene))
        .collect();
    bar.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Render pixels using a single thread.
#[allow(clippy::option_expect_used)]
#[allow(clippy::result_expect_used)]
#[inline]
fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input, scene: &Scene) -> Output {
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    let mut data = Output::new(
        [width, height],
        input.cols.map()["scale"].clone(),
        Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 0.0),
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
        ]),
        Gradient::new(vec![
            LinSrgba::new(0.0, 0.0, 0.0, 1.0),
            LinSrgba::new(1.0, 1.0, 1.0, 1.0),
        ]),
        [
            Gradient::new(vec![
                LinSrgba::new(0.0, 0.0, 0.0, 0.0),
                LinSrgba::new(1.0, 0.0, 0.0, 1.0),
            ]),
            Gradient::new(vec![
                LinSrgba::new(0.0, 0.0, 0.0, 0.0),
                LinSrgba::new(0.0, 1.0, 0.0, 1.0),
            ]),
            Gradient::new(vec![
                LinSrgba::new(0.0, 0.0, 0.0, 0.0),
                LinSrgba::new(0.0, 0.0, 1.0, 1.0),
            ]),
        ],
    );

    let super_samples = scene.cam().sensor().super_samples();
    let dof_samples = scene.cam().focus().dof_samples();
    let h_res = scene.cam().sensor().res().0;

    let weight = 1.0 / f64::from(super_samples * dof_samples);

    let mut rng = thread_rng();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let now = std::time::Instant::now();

            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let mut col = palette::LinSrgba::default();
            let mut first_hits = Vec::with_capacity((super_samples * dof_samples) as usize);
            let mut last_hits = Vec::with_capacity((super_samples * dof_samples) as usize);

            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);

                    let (c, first_hit_index, last_hit_index) =
                        paint(&mut rng, input, scene, ray, 1.0);
                    col += c * weight as f32;
                    first_hits.push(first_hit_index);
                    last_hits.push(last_hit_index);
                }
            }

            // Colouring.
            {
                data.image[pixel] = col;
            }

            // First hit.
            {
                let mut indices = Vec::with_capacity(first_hits.len());
                let mut total_norm = Vec3::new(0.0, 0.0, 0.0);
                for fh in &first_hits {
                    if let Some(hit) = fh {
                        indices.push(hit.index as i32);
                        total_norm += hit.norm.into_inner();
                    } else {
                        indices.push(-1);
                    }
                }
                data.first_hit_index[pixel] =
                    crate::sci::math::sort::order::mode(&indices).unwrap();
                data.first_hit_norm[pixel] = if total_norm.magnitude() > 1.0e-3 {
                    Some(Dir3::new_normalize(total_norm))
                } else {
                    None
                };
            }

            // Last hit.
            {
                let mut indices = Vec::with_capacity(last_hits.len());
                let mut total_norm = Vec3::new(0.0, 0.0, 0.0);
                for lh in &last_hits {
                    if let Some(hit) = lh {
                        indices.push(hit.index as i32);
                        total_norm += hit.norm.into_inner();
                    } else {
                        indices.push(-1);
                    }
                }
                data.last_hit_index[pixel] = crate::sci::math::sort::order::mode(&indices).unwrap();
                data.last_hit_norm[pixel] = if total_norm.magnitude() > 1.0e-3 {
                    Some(Dir3::new_normalize(total_norm))
                } else {
                    None
                };
            }

            // Time.
            {
                let time = std::time::Instant::now().duration_since(now).as_nanos();
                data.time[pixel] += time as f64;
            }
        }
    }

    data
}
