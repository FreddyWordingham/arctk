//! Simulation control functions.

use super::{Cloud, Data, Gradient, Settings};
use crate::{
    err::Error,
    geom::Grid,
    math::Vec3,
    ord::{X, Y},
    tools::{linear_to_three_dim, ProgressBar, SilentProgressBar},
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use rand::thread_rng;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Run a multi-threaded diffusion simulation.
/// # Errors
/// if the progress bad can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(cloud: &Cloud, values: &Array3<f64>) -> Result<Data, Error> {
    debug_assert!(values.shape() == cloud.grid.res());

    let pb = SilentProgressBar::new(cloud.grid.total_cells() as u64);
    let pb = Arc::new(Mutex::new(pb));

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let mut out: Vec<_> = threads
        .par_iter()
        .map(|_id| {
            thread(
                &Arc::clone(&pb),
                cloud.sett.block_size(),
                cloud.grid,
                values,
                cloud.coeffs,
            )
        })
        .collect();

    let mut data = out.pop().expect("No data received.");
    while let Some(o) = out.pop() {
        data += &o;
    }

    Ok(data)
}

/// Run a diffusion simulation using a single thread.
#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn single_thread(cloud: &Cloud, values: &Array3<f64>) -> Result<Data, Error> {
    let pb = SilentProgressBar::new(cloud.grid.total_cells() as u64);
    let pb = Arc::new(Mutex::new(pb));

    Ok(thread(
        &pb,
        cloud.sett.block_size(),
        cloud.grid,
        values,
        cloud.coeffs,
    ))
}

/// Thread control function.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn thread(
    pb: &Arc<Mutex<SilentProgressBar>>,
    block_size: u64,
    grid: &Grid,
    values: &Array3<f64>,
    coeffs: &Array3<f64>,
) -> Data {
    let rates = Array3::zeros(*grid.res());
    let voxel_size = grid.voxel_size();
    let voxel_size_sq = Vec3::new(
        voxel_size.x * voxel_size.x,
        voxel_size.y * voxel_size.y,
        voxel_size.z * voxel_size.z,
    );

    diff_rate(pb, block_size, &voxel_size_sq, values, coeffs, rates)
}

/// Calculate the diffusion rates for each cell.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn diff_rate(
    pb: &Arc<Mutex<SilentProgressBar>>,
    block_size: u64,
    cell_size_sq: &Vec3,
    values: &Array3<f64>,
    coeffs: &Array3<f64>,
    mut rate: Array3<f64>,
) -> Data {
    debug_assert!(values.shape() == coeffs.shape());

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

            let stencil = Gradient::new(index, values);
            let r = stencil.rate(coeffs[index], cell_size_sq);
            rate[index] = r;
        }
    }

    rate
}
