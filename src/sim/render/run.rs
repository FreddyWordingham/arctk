//! Fast-scheme rendering function.

use crate::{
    render::{Input, Output, Shader},
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
pub fn run(input: &Input, shader: &Shader) -> Result<Output, Error> {
    let num_pixels = shader.cam().sensor().num_pixels();
    let bar = Bar::new("Rendering", num_pixels as u64);
    let bar = Arc::new(Mutex::new(bar));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut out: Vec<Output> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&bar), input, shader))
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
fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input, shader: &Shader) -> Output {
    let width = shader.cam().sensor().res().0 as usize;
    let height = shader.cam().sensor().res().1 as usize;

    let super_samples = shader.cam().sensor().super_samples();
    let dof_samples = shader.cam().focus().dof_samples();
    let h_res = shader.cam().sensor().res().0;

    let weight = 1.0 / f64::from(super_samples * dof_samples);

    let mut rng = thread_rng();

    let mut data = Output::new([width, height]);
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let mut total_col = palette::LinSrgba::default();
            for sub_sample in 0..super_samples {
                let offset = rng.gen_range(0.0, 2.0 * PI);
                for depth_sample in 0..dof_samples {
                    let ray = shader
                        .cam()
                        .gen_ray(pixel, offset, sub_sample, depth_sample);

                    let col = paint(&mut rng, input, shader, ray, 1.0);
                    total_col += col * weight as f32;
                }
            }

            data.image[pixel] = col;
        }
    }

    data
}
