//! Simulation control functions.

use crate::{
    err::Error,
    fs::Save,
    math::Vec3,
    sim::diffuse::{stencil, Input},
    tools::{ProgressBar, SilentProgressBar},
};
use ndarray::{Array1, Array3};
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

/// Run a single-threaded Diffuse simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn single_thread(
    input: &Input,
    mut values: Array3<f64>,
    out_dir: &PathBuf,
) -> Result<Array3<f64>, Error> {
    // Constants.
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
    let max_dt = min_voxel_size_sq / (8.0 * max_coeff);
    let dt = max_dt * (1.0 - input.sett.quality()).min(1.0).max(0.0);

    let steps = 1 + input.sett.dumps();
    let step_time = input.sett.time() / steps as f64;

    // Allocation.
    let mut rates = Array3::zeros(*input.grid.res());

    // Initial value write.
    values.save(&out_dir.join(&format!("{:03}_diff.nc", 0)))?;
    rates.save(&out_dir.join(&format!("{:03}_rate.nc", 0)))?;

    // Time loop.
    for n in 0..steps {
        let vr = diffuse(input, &voxel_size_sq, step_time, dt, values, rates);
        values = vr.0;
        rates = vr.1;

        values.save(&out_dir.join(&format!("{:03}_diff.nc", n + 1)))?;
        rates.save(&out_dir.join(&format!("{:03}_rate.nc", n + 1)))?;
    }

    Ok(values)
}

/// Integrate forward a given amount of time.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn diffuse(
    input: &Input,
    voxel_size_sq: &Vec3,
    time: f64,
    mut dt: f64,
    mut values: Array3<f64>,
    rates: Array3<f64>,
) -> (Array3<f64>, Array3<f64>) {
    debug_assert!(time > 0.0);
    debug_assert!(dt > 0.0);

    // Constants.
    let steps = 1 + (time / dt) as usize;
    dt = time / steps as f64;

    // Threading.
    let rates = Mutex::new(rates);
    let threads: Vec<_> = (0..num_cpus::get()).collect();

    // Evolution.
    let mut pb = ProgressBar::new("Diffusing", steps);
    for _n in 0..steps {
        // Calculate diffusion rates.
        let spb = Arc::new(Mutex::new(SilentProgressBar::new(values.len())));
        let _out: Vec<_> = threads
            .par_iter()
            .map(|_id| calc_diffuse_rates(input, voxel_size_sq, &values, &rates, &Arc::clone(&spb)))
            .collect();

        // Apply diffusion.
        values += &(&(*rates.lock().expect("Could not lock rates array.")) * dt);

        // Apply source terms.
        values += &(input.sources * dt);

        // Check for zero.
        values.mapv_inplace(|x| x.max(0.0));

        // Tick.
        pb.tick();
    }

    let rates = rates.into_inner().expect("Failed to unwrap rates array.");

    (values, rates)
}

/// Calculate the diffusion rates.
#[allow(clippy::expect_used)]
#[inline]
fn calc_diffuse_rates(
    input: &Input,
    voxel_size_sq: &Vec3,
    values: &Array3<f64>,
    rates: &Mutex<Array3<f64>>,
    pb: &Arc<Mutex<SilentProgressBar>>,
) {
    // Constants.
    let res = *input.grid.res();
    let block_size = input.sett.block_size();

    // Allocation.
    let mut holder = Array1::zeros(block_size);

    // Rate calculations.
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        // Calculate rates.
        for n in start..end {
            let index = crate::tools::index::linear_to_three_dim(n, &res);
            let stencil = stencil::Grad::new(index, values);
            holder[n - start] = stencil.rate(input.coeffs[index], voxel_size_sq);
        }

        // Store rates.
        {
            let mut rates = rates.lock().expect("Could not lock rate array.");
            for n in start..end {
                let index = crate::tools::index::linear_to_three_dim(n, &res);
                rates[index] = holder[n - start];
            }
        }
    }
}
