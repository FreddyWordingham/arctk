//! Simulation control functions.

use crate::{
    chem::Reactor,
    err::Error,
    fs::Save,
    math::Vec3,
    sim::reactor::{stencil, Input},
    tools::ProgressBar,
};
use ndarray::{Array1, Array4, Axis};
use ndarray_stats::QuantileExt;
use std::f64::MIN_POSITIVE;
use std::path::PathBuf;

/// Run a single-threaded reaction-diffusion simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn single_thread(
    out_dir: &PathBuf,
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
    for n in 0..steps {
        let vr = integrate(input, values, rates, &voxel_size_sq, step_time, dt);
        values = vr.0;
        rates = vr.1;

        for (name, si) in input.specs.set().map() {
            values
                .index_axis(Axis(0), *si)
                .save(&out_dir.join(&format!("{:03}_{}_diff.nc", name, n)))?;
        }
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
        // First reaction half-step.
        values = react(input, values, dt / 2.0);

        // Full diffusion step.
        rates = calc_diff_rates(input, &values, rates, voxel_size_sq);
        values += &(&rates * dt);

        // Last reaction half-step.
        values = react(input, values, dt / 2.0);

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

/// Reaction calculation function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn react(input: &Input, mut values: Array4<f64>, time: f64) -> Array4<f64> {
    debug_assert!(time > 0.0);

    let [rs, rx, ry, rz] = [
        values.shape()[0],
        values.shape()[1],
        values.shape()[2],
        values.shape()[3],
    ];

    let mut concs = Array1::zeros(rs);
    for xi in 0..rx {
        for yi in 0..ry {
            for zi in 0..rz {
                for si in 0..rs {
                    concs[si] = values[[si, xi, yi, zi]] + MIN_POSITIVE;
                }

                concs = evolve_rk4(
                    input.reactor,
                    concs,
                    time,
                    input.sett.quality(),
                    input.sett.min_time(),
                );

                for si in 0..rs {
                    values[[si, xi, yi, zi]] = concs[si] - MIN_POSITIVE;
                }
            }
        }
    }

    values
}

/// Evolve forward the given amount of time.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn evolve_rk4(
    reactor: &Reactor,
    mut concs: Array1<f64>,
    total_time: f64,
    quality: f64,
    min_time: f64,
) -> Array1<f64> {
    debug_assert!(total_time > 0.0);
    debug_assert!(quality > 0.0);
    debug_assert!(quality < 1.0);
    debug_assert!(min_time <= total_time);

    let mut time = 0.0;
    let mut k1;
    let mut k2;
    let mut k3;
    let mut k4;
    while time < total_time {
        k1 = reactor.deltas(&concs);

        let dt = ((&concs / &k1)
            .max()
            .expect("Failed to determine minimum rate of change.")
            * quality)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        k2 = reactor.deltas(&(&concs + &(&k1 * half_dt)));
        k3 = reactor.deltas(&(&concs + &(&k2 * half_dt)));
        k4 = reactor.deltas(&(&concs + &(&k3 * dt)));

        concs += &(&(&k1 + &(2.0 * (k2 + k3)) + &k4) * sixth_dt);
        time += dt;
    }

    concs
}
