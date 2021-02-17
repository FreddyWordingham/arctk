//! Simulation control functions.

use crate::{
    chem::Reactor,
    err::Error,
    fs::Save,
    math::Vec3,
    ord::{X, Y, Z},
    sim::reactor::{stencil, Input},
    tools::ProgressBar,
};
use ndarray::{Array1, Array4, Axis};
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use std::{f64::MIN_POSITIVE, path::PathBuf};

/// Run a single-threaded reaction-diffusion simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(
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
        let vr = integrate(input, values, rates, &voxel_size_sq, step_time, dt)?;
        values = vr.0;
        rates = vr.1;

        for (name, si) in input.specs.set().map() {
            values
                .index_axis(Axis(0), *si)
                .save(&out_dir.join(&format!("{:03}_{}_diff.nc", n, name)))?;
        }
    }

    Ok(values)
}

/// Integrate forward a given amount of time.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn integrate(
    input: &Input,
    mut values: Array4<f64>,
    mut rates: Array4<f64>,
    voxel_size_sq: &Vec3,
    time: f64,
    tar_dt: f64,
) -> Result<(Array4<f64>, Array4<f64>), Error> {
    debug_assert!(time > 0.0);
    debug_assert!(tar_dt > 0.0);

    let num_steps = (time / tar_dt) as usize;
    let dt = time / num_steps as f64;

    let mut pb = ProgressBar::new("Diffusion-Reaction", num_steps);
    for _ in 0..num_steps {
        // First reaction half-step.
        values = react(input, values, dt / 2.0)?;

        // Full diffusion step.
        let values_rates = diffuse(input, values, rates, voxel_size_sq, dt)?;
        values = values_rates.0;
        rates = values_rates.1;
        // Potentially check for -ve values here (from sink terms).

        // Last reaction half-step.
        values = react(input, values, dt / 2.0)?;

        pb.tick();
    }
    pb.finish_with_message("Step complete.");

    Ok((values, rates))
}

/// Diffusion rate calculation function.
#[allow(clippy::expect_used)]
#[inline]
fn diffuse(
    input: &Input,
    mut values: Array4<f64>,
    rates: Array4<f64>,
    voxel_size_sq: &Vec3,
    time: f64,
) -> Result<(Array4<f64>, Array4<f64>), Error> {
    let pb = ProgressBar::new("Multi-threaded", values.len() / input.specs.len());
    let pb = Arc::new(Mutex::new(pb));

    let rates = Mutex::new(rates);

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let _out: Vec<_> = threads
        .par_iter()
        .map(|_id| {
            diffuse_impl(
                input,
                &values,
                &rates,
                voxel_size_sq,
                time,
                &Arc::clone(&pb),
            )
        })
        .collect();
    // pb.lock()?.finish_with_message("Simulation complete.");

    let rates = rates.into_inner().expect("Failed to unwrap rates array.");

    values += &(&rates * time);

    Ok((values, rates))
}

/// Diffusion rate calculation function.
#[allow(clippy::expect_used)]
#[inline]
fn diffuse_impl(
    input: &Input,
    values: &Array4<f64>,
    rates: &Mutex<Array4<f64>>,
    voxel_size_sq: &Vec3,
    time: f64,
    pb: &Arc<Mutex<ProgressBar>>,
) {
    debug_assert!(time > 0.0);

    let [rs, rx, ry, _rz] = [
        values.shape()[0],
        values.shape()[1],
        values.shape()[2],
        values.shape()[3],
    ];

    let block_size = input.sett.block_size();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n % rx;
            let yi = (n / rx) % ry;
            let zi = n / (rx * ry);

            for si in 0..rs {
                let index = [si, xi, yi, zi];
                let stencil = stencil::Grad::new(index, values);
                rates.lock().expect("Could not lock rate array.")[index] =
                    stencil.rate(input.coeffs[index], voxel_size_sq) + input.sources[index];
            }
        }
    }
}

/// Reaction calculation function.
#[allow(clippy::expect_used)]
#[inline]
fn react(input: &Input, values: Array4<f64>, time: f64) -> Result<Array4<f64>, Error> {
    debug_assert!(time > 0.0);

    let pb = ProgressBar::new("Multi-threaded", values.len() / input.specs.len());
    let pb = Arc::new(Mutex::new(pb));

    let values = Mutex::new(values);

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let _out: Vec<_> = threads
        .par_iter()
        .map(|_id| react_impl(input, &values, time, &Arc::clone(&pb)))
        .collect();

    Ok(values.into_inner().expect("Failed to unwrap values array."))
}

/// Reaction calculation function.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn react_impl(input: &Input, values: &Mutex<Array4<f64>>, time: f64, pb: &Arc<Mutex<ProgressBar>>) {
    debug_assert!(time > 0.0);

    let [rs, rx, ry, rz] = [
        input.specs.len(),
        input.grid.res()[X],
        input.grid.res()[Y],
        input.grid.res()[Z],
    ];

    let mut concs = Array1::zeros(rs);
    let mut ks = [concs.clone(), concs.clone(), concs.clone(), concs.clone()];

    let block_size = input.sett.block_size();
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        for n in start..end {
            let xi = n % rx;
            let yi = (n / rx) % ry;
            let zi = n / (rx * ry);

            for si in 0..rs {
                concs[si] = values.lock().expect("Could not lock values array.")[[si, xi, yi, zi]]
                    + MIN_POSITIVE;
            }

            let concs_ks = evolve_rk4(
                input.reactor,
                concs,
                ks,
                time * input.multipliers[[xi, yi, zi]],
                input.sett.quality(),
                input.sett.min_time(),
            );
            concs = concs_ks.0;
            ks = concs_ks.1;

            for si in 0..rs {
                values.lock().expect("Could not lock values array.")[[si, xi, yi, zi]] =
                    concs[si] - MIN_POSITIVE;
            }
        }
    }
}

/// Evolve forward the given amount of time.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn evolve_rk4(
    reactor: &Reactor,
    mut concs: Array1<f64>,
    mut ks: [Array1<f64>; 4],
    total_time: f64,
    quality: f64,
    min_time: f64,
) -> (Array1<f64>, [Array1<f64>; 4]) {
    debug_assert!(total_time > 0.0);
    debug_assert!(quality > 0.0);
    debug_assert!(quality < 1.0);
    debug_assert!(min_time <= total_time);

    let mut time = 0.0;
    while time < total_time {
        ks[0] = reactor.deltas(&concs);

        let dt = ((&concs / &ks[0])
            .max()
            .expect("Failed to determine minimum rate of change.")
            * quality)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        ks[1] = reactor.deltas(&(&concs + &(&ks[0] * half_dt)));
        ks[2] = reactor.deltas(&(&concs + &(&ks[1] * half_dt)));
        ks[3] = reactor.deltas(&(&concs + &(&ks[2] * dt)));

        concs += &(&(&ks[0] + &(2.0 * (&ks[1] + &ks[2])) + &ks[3]) * sixth_dt);
        time += dt;
    }

    (concs, ks)
}
