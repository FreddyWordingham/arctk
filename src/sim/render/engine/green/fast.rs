//! Live-scheme rendering function.

use super::{paint, Output};
use crate::{
    render::{Input, Scene},
    Bar, Error, Vec3,
};
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

    let mut data = out.pop().expect("No data recieved.");
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

    let mut data = Output::new([width, height]);

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
            let mut ave_first_norm = Vec3::new(0.0, 0.0, 0.0);
            let mut ave_first_dist = 0.0;
            let mut last_hits = Vec::with_capacity((super_samples * dof_samples) as usize);
            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = scene.cam().gen_ray(pixel, offset, sub_sample, depth_sample);
                    let (c, fh, fd, first_norm, lh) = paint(&mut rng, input, scene, ray, 1.0);
                    col += c * weight as f32;
                    ave_first_norm += first_norm * weight;
                    ave_first_dist += fd * weight;
                    first_hits.push(fh);
                    last_hits.push(lh);
                }
            }
            data.image[pixel] += col;
            data.first_hit[pixel] = mode(&first_hits).expect("Could not determine first hit.");
            data.first_dist[pixel] = ave_first_dist;
            data.first_norm[pixel] = ave_first_norm;
            data.last_hit[pixel] = mode(&last_hits).expect("Could not determine last hit.");

            let time = std::time::Instant::now().duration_since(now).as_nanos();
            data.time[pixel] += time as f64;
        }
    }

    data
}

/// Get the mode of a slice.
#[inline]
#[must_use]
fn mode(numbers: &[usize]) -> Option<usize> {
    let mut counts = std::collections::HashMap::new();

    numbers.iter().copied().max_by_key(|&n| {
        let count = counts.entry(n).or_insert(0);
        *count += 1;
        *count
    })
}
