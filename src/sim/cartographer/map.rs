//! Mapping function.

use crate::{
    cartographer::{Input, Output},
    Bar, Error, Trace, X, Y, Z,
};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Map a volume of surfaces.
/// # Errors
/// if a mutex unwrapping failed or
/// an arc unwrapping failed.
#[allow(clippy::option_expect_used)]
#[inline]
#[must_use]
pub fn map(input: &Input) -> Result<Output, Error> {
    let res = *input.grid.res();
    let num_cells = res[X] * res[Y] * res[Z];
    let bar = Bar::new("Mapping", num_cells as u64);
    let bar = Arc::new(Mutex::new(bar));

    let threads: Vec<usize> = (0..num_cpus::get()).collect();
    let mut out: Vec<Output> = threads
        .par_iter()
        .map(|_id| run_thread(&Arc::clone(&bar), input))
        .collect();
    bar.lock()?.finish_with_message("Render complete.");

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Map a volume of surfaces using a single thread.
#[allow(clippy::option_expect_used)]
#[allow(clippy::result_expect_used)]
#[inline]
#[must_use]
pub fn run_thread(pb: &Arc<Mutex<Bar>>, input: &Input) -> Output {
    let res = *input.grid.res();
    let data = Output::new(res);

    let bump_dist = input.sett.bump_dist();

    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(input.sett.block_size());
        std::mem::drop(pb);
        b
    } {
        for i in start..end {
            // Get the cell index.
            let n = i as usize;
            let xi = n % res[X];
            let yi = (n / res[X]) % res[Y];
            let zi = n / (res[X] * res[Y]);
            let index = [xi, yi, zi];

            // Get the voxel.
            let voxel = input.grid.gen_voxel(&index);

            // Create a ray from the voxel centre.
            let centre = voxel.centre();
            let ray = input.sett.cast().gen_ray(centre);

            // Determine what the ray observes.
            let bound_dist = input
                .grid
                .boundary()
                .dist(&ray)
                .expect("Could not determine voxel distance.");

            if let Some(hit) = input.tree.observe(ray.clone(), bump_dist, bound_dist) {
                println!("{}\t{}", hit.side().is_inside(), hit.group());
            } else {
                println!("[WARN]: Did not observe surface with ray: {}", &ray);
            }
        }
    }

    data
}
