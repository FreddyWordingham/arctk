//! Simulation control functions.

use crate::{
    err::Error,
    math::Vec3,
    sim::reactor::{stencil, Input},
    tools::ProgressBar,
};
use ndarray::Array4;
use ndarray_stats::QuantileExt;
use std::path::PathBuf;

/// Run a single-threaded reaction-diffusion simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn single_thread(
    _out_dir: &PathBuf,
    input: &Input,
    mut values: Array4<f64>,
) -> Result<Array4<f64>, Error> {
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
    let max_diff_dt = min_voxel_size_sq / (8.0 * max_coeff);
    let dt = max_diff_dt * (1.0 - input.sett.quality()).min(1.0).max(0.0);

    let steps = input.sett.dumps();
    let step_time = input.sett.time() / steps as f64;
    let mut rates = Array4::zeros(values.raw_dim());
    for _n in 0..steps {
        let vr = integrate(input, values, rates, &voxel_size_sq, step_time, dt);
        values = vr.0;
        rates = vr.1;
        // values.save(&out_dir.join(&format!("{:03}_diff.nc", n)))?;
        // rates.save(&out_dir.join(&format!("{:03}_rate.nc", n)))?;
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
    mut values: Array4<f64>,
    mut rates: Array4<f64>,
    voxel_size_sq: &Vec3,
    time: f64,
    tar_dt: f64,
) -> (Array4<f64>, Array4<f64>) {
    debug_assert!(time > 0.0);
    debug_assert!(tar_dt > 0.0);

    let num_steps = (time / tar_dt) as usize;
    let dt = time / num_steps as f64;

    let mut pb = ProgressBar::new("Diffusion-Reaction", num_steps);
    for _ in 0..num_steps {
        rates = calc_diff_rates(input, &values, rates, voxel_size_sq);
        values += &(&rates * dt);
        pb.tick();
    }
    pb.finish_with_message("Simulation complete.");

    (values, rates)
}

/// Diffusion rate calculation function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn calc_diff_rates(
    input: &Input,
    values: &Array4<f64>,
    mut rates: Array4<f64>,
    voxel_size_sq: &Vec3,
) -> Array4<f64> {
    let [rs, rx, ry, rz] = [
        values.shape()[0],
        values.shape()[1],
        values.shape()[2],
        values.shape()[3],
    ];

    for si in 0..rs {
        for zi in 0..rz {
            for yi in 0..ry {
                for xi in 0..rx {
                    let index = [si, xi, yi, zi];

                    // let stencil = stencil::Reflect::new(index, values);
                    let stencil = stencil::Grad::new(index, values);
                    rates[index] = stencil.rate(input.coeffs[index], voxel_size_sq);
                }
            }
        }
    }

    rates
}
