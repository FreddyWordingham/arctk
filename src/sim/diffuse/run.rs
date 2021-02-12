//! Simulation control functions.

use crate::{
    err::Error,
    fs::Save,
    math::Vec3,
    ord::{X, Y},
    sim::diffuse::{stencil, Input},
    tools::ProgressBar,
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::path::PathBuf;

/// Run a single-threaded Diffuse simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn single_thread(
    out_dir: &PathBuf,
    input: &Input,
    mut values: Array3<f64>,
) -> Result<Array3<f64>, Error> {
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

    let steps = input.sett.dumps();
    let step_time = input.sett.time() / steps as f64;
    let mut rates = Array3::zeros(*input.grid.res());
    for n in 0..steps {
        let vr = integrate(input, values, rates, &voxel_size_sq, step_time, dt);
        values = vr.0;
        rates = vr.1;
        values.save(&out_dir.join(&format!("{:03}_diff.nc", n)))?;
        rates.save(&out_dir.join(&format!("{:03}_rate.nc", n)))?;
    }

    Ok(values)
}

/// Integrate forward a given amount of time.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn integrate(
    input: &Input,
    mut values: Array3<f64>,
    mut rates: Array3<f64>,
    voxel_size_sq: &Vec3,
    time: f64,
    tar_dt: f64,
) -> (Array3<f64>, Array3<f64>) {
    debug_assert!(time > 0.0);
    debug_assert!(tar_dt > 0.0);

    let num_steps = (time / tar_dt) as usize;
    let dt = time / num_steps as f64;

    let mut pb = ProgressBar::new("Diffusing", num_steps);
    for _ in 0..num_steps {
        rates = calc_rates(input, &values, rates, voxel_size_sq);
        values += &(&rates * dt);
        // Potentially check for -ve values here.
        pb.tick();
    }
    pb.finish_with_message("Step complete.");

    (values, rates)
}

/// Rate calculation function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn calc_rates(
    input: &Input,
    values: &Array3<f64>,
    mut rates: Array3<f64>,
    voxel_size_sq: &Vec3,
) -> Array3<f64> {
    let res = *input.grid.res();

    for n in 0..values.len() {
        let xi = n % res[X];
        let yi = (n / res[X]) % res[Y];
        let zi = n / (res[X] * res[Y]);

        let index = [xi, yi, zi];

        let stencil = stencil::Grad::new(index, values);
        rates[index] = stencil.rate(input.coeffs[index], voxel_size_sq);
    }

    rates
}
