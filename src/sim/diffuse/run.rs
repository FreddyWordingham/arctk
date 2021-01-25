//! Simulation control functions.

use crate::{
    err::Error,
    math::Vec3,
    ord::{X, Y},
    report,
    sim::diffuse::{stencil::Grad, Input},
    tools::{ProgressBar, SilentProgressBar},
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded Diffuse simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(input: &Input, mut values: Array3<f64>) -> Result<Array3<f64>, Error> {
    let spb = SilentProgressBar::new(input.grid.num_cells());
    let spb = Arc::new(Mutex::new(spb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();

    let voxel_size = input.grid.voxel_size();
    let voxel_size_sq = Vec3::new(
        voxel_size.x * voxel_size.x,
        voxel_size.y * voxel_size.y,
        voxel_size.z * voxel_size.z,
    );
    let min_voxel_size_sq = voxel_size_sq.min();
    report!(min_voxel_size_sq);

    let max_coeff = input
        .coeffs
        .max()
        .expect("Failed to determine maximum coefficient.");
    report!(max_coeff);
    let max_dt = min_voxel_size_sq / (4.0 * max_coeff * max_coeff);
    report!(max_dt);

    let dt = max_dt * 0.1;
    let num_steps = (input.sett.time() / dt) as usize;
    let dt = input.sett.time() / num_steps as f64;
    report!(dt);
    let mut pb = ProgressBar::new("Diffusing", num_steps);
    for _n in 0..10 {
        let mut out: Vec<_> = threads
            .par_iter()
            .map(|_id| thread(input, &values, &Arc::clone(&spb)))
            .collect();

        let mut rates = out.pop().expect("No data received.");
        while let Some(o) = out.pop() {
            rates += &o;
        }

        {
            let index = [15, 15, 15];
            let stencil = Grad::new(index, &values);
            let rate = stencil.rate(input.coeffs[index], &voxel_size_sq);

            report!(stencil);
            report!(rate);
            report!(rates);
            report!(rates[[15, 15, 15]]);
            report!(values[[15, 15, 15]]);
        }

        values += &(&rates * dt);
        pb.tick();
        // panic!("TETS");
    }
    pb.finish_with_message("Simulation complete.");

    Ok(values)
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(input: &Input, values: &Array3<f64>, spb: &Arc<Mutex<SilentProgressBar>>) -> Array3<f64> {
    let res = *input.grid.res();
    let voxel_size = input.grid.voxel_size();
    let voxel_size_sq = Vec3::new(
        voxel_size.x * voxel_size.x,
        voxel_size.y * voxel_size.y,
        voxel_size.z * voxel_size.z,
    );

    let block_size = input.sett.block_size();

    let mut rate = Array3::zeros(res);
    while let Some((start, end)) = {
        let mut spb = spb.lock().expect("Could not lock progress bar.");
        let b = spb.block(block_size);
        std::mem::drop(spb);
        b
    } {
        for n in start as usize..end as usize {
            let xi = n % res[X];
            let yi = (n / res[X]) % res[Y];
            let zi = n / (res[X] * res[Y]);

            let index = [xi, yi, zi];

            let stencil = Grad::new(index, values);
            rate[index] = stencil.rate(input.coeffs[index], &voxel_size_sq);
        }
    }

    rate
}
