//! Simulation control functions.

use crate::{
    err::Error,
    math::Vec3,
    ord::{X, Y},
    sim::diffuse::{stencil::Grad, Input},
    tools::SilentProgressBar,
};
use ndarray::Array3;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded Diffuse simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(input: &Input, values: Array3<f64>) -> Result<Array3<f64>, Error> {
    let pb = SilentProgressBar::new(input.grid.num_cells());
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| thread(input, &values, &Arc::clone(&pb)))
        .collect();

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a MCRT simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn single_thread(input: &Input, values: Array3<f64>) -> Result<Array3<f64>, Error> {
    let pb = SilentProgressBar::new(input.grid.num_cells());
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(input, &values, &pb))
}

/// Thread control function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(input: &Input, values: &Array3<f64>, pb: &Arc<Mutex<SilentProgressBar>>) -> Array3<f64> {
    let rates = Array3::zeros(*input.grid.res());
    let voxel_size = input.grid.voxel_size();
    let voxel_size_sq = Vec3::new(
        voxel_size.x * voxel_size.x,
        voxel_size.y * voxel_size.y,
        voxel_size.z * voxel_size.z,
    );

    diff_rate(
        pb,
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
    pb: &Arc<Mutex<SilentProgressBar>>,
    block_size: usize,
    cell_size_sq: &Vec3,
    values: &Array3<f64>,
    coeffs: &Array3<f64>,
    mut rate: Array3<f64>,
) -> Array3<f64> {
    let res = values.shape();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
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
