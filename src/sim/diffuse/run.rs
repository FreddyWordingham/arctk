//! Simulation control functions.

use crate::{
    err::Error,
    fs::Save,
    math::Vec3,
    ord::{X, Y},
    sim::diffuse::{stencil::Grad, Input},
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
    let steps = input.sett.dumps();
    let step_time = input.sett.time() / steps as f64;
    for n in 0..steps {
        values = thread(input, values, step_time)?;
        values.save(&out_dir.join(&format!("{:03}_diff.nc", n)))?;
    }

    Ok(values)
}

/// Run a single-threaded Diffuse simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn thread(input: &Input, mut values: Array3<f64>, time: f64) -> Result<Array3<f64>, Error> {
    debug_assert!(time > 0.0);

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

    let dt = max_dt * (1.0 - input.sett.quality());
    let num_steps = (time / dt) as usize;
    let dt = time / num_steps as f64;

    let mut pb = ProgressBar::new("Diffusing", num_steps);
    let mut rate = Array3::zeros(*input.grid.res());
    for _ in 0..num_steps {
        rate = rates(input, &values, rate);
        values += &(&rate * dt);
        pb.tick();
    }
    pb.finish_with_message("Simulation complete.");

    Ok(values)
}

/// Rate calculation function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn rates(input: &Input, values: &Array3<f64>, mut rates: Array3<f64>) -> Array3<f64> {
    let res = *input.grid.res();
    let voxel_size = input.grid.voxel_size();
    let voxel_size_sq = Vec3::new(
        voxel_size.x * voxel_size.x,
        voxel_size.y * voxel_size.y,
        voxel_size.z * voxel_size.z,
    );

    let length = values.len();
    for n in 0..length {
        let xi = n % res[X];
        let yi = (n / res[X]) % res[Y];
        let zi = n / (res[X] * res[Y]);

        let index = [xi, yi, zi];

        let stencil = Grad::new(index, values);
        rates[index] = stencil.rate(input.coeffs[index], &voxel_size_sq);
    }

    rates
}
