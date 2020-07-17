//! Live-scheme rendering function.

// use super::{paint, Output};
use super::Output;
use crate::{
    render::{Input, Scene},
    Bar, Error,
};
// use minifb::{CursorStyle, Scale, ScaleMode, Window, WindowOptions};
// use palette::Pixel;
// use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

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

    let data = out.pop().expect("No data recieved.");
    // while if let Some(o) = out.pop() {
    //     data += o;
    // }

    Ok(data)
}

/// Render pixels using a single thread.
#[allow(clippy::result_expect_used)]
#[inline]
fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input, scene: &Scene) -> Output {
    let width = scene.cam().sensor().res().0 as usize;
    let height = scene.cam().sensor().res().1 as usize;

    let mut data = Output::new([width, height]);

    // let mut rng = thread_rng();
    let super_samples = scene.cam().sensor().super_samples();
    let dof_samples = scene.cam().focus().dof_samples();
    let h_res = scene.cam().sensor().res().0;

    // let weight = 1.0 / f64::from(super_samples * dof_samples);

    if let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for p in start..end {
            let now = std::time::Instant::now();

            let pixel = [(p % h_res) as usize, (p / h_res) as usize];

            let time = std::time::Instant::now().duration_since(now).as_nanos();
            data.time[pixel] += time as f64;
        }
    }

    data
}
