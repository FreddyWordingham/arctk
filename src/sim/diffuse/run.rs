//! Simulation control functions.

use crate::{
    err::Error,
    math::Vec3,
    ord::{X, Y},
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

    let max_coeff = input
        .coeffs
        .max()
        .expect("Failed to determine maximum coefficient.");
    let max_dt = min_voxel_size_sq / (4.0 * max_coeff * max_coeff);

    let dt = max_dt * 0.1;
    let num_steps = (input.sett.time() / dt) as usize;
    let mut pb = ProgressBar::new("Diffusing", num_steps);
    for _n in 0..num_steps {
        let mut out: Vec<_> = threads
            .par_iter()
            .map(|_id| thread(input, &values, &Arc::clone(&spb)))
            .collect();

        let mut rates = out.pop().expect("No data received.");
        while let Some(o) = out.pop() {
            rates += &o;
        }

        values += &(&rates * dt);
        pb.tick();
    }
    pb.finish_with_message("Simulation complete.");

    Ok(values)
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(input: &Input, values: &Array3<f64>, spb: &Arc<Mutex<SilentProgressBar>>) -> Array3<f64> {
    let rates = Array3::zeros(*input.grid.res());
    let voxel_size = input.grid.voxel_size();
    let voxel_size_sq = Vec3::new(
        voxel_size.x * voxel_size.x,
        voxel_size.y * voxel_size.y,
        voxel_size.z * voxel_size.z,
    );

    diff_rate(
        spb,
        input.sett.block_size(),
        &voxel_size_sq,
        values,
        input.coeffs,
        rates,
    )
}

/// Calculate the diffusion rates for each cell.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn diff_rate(
    spb: &Arc<Mutex<SilentProgressBar>>,
    block_size: usize,
    cell_size_sq: &Vec3,
    values: &Array3<f64>,
    coeffs: &Array3<f64>,
    mut rate: Array3<f64>,
) -> Array3<f64> {
    let res = values.shape();
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
            let r = stencil.rate(coeffs[index], cell_size_sq);
            rate[index] = r;
        }
    }

    rate
}
