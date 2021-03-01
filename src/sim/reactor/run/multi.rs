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
use ndarray::{Array1, Array2, Array4, Axis};
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::{
    f64::MIN_POSITIVE,
    path::PathBuf,
    sync::{Arc, Mutex},
};

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
    mut swap: Array4<f64>,
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
        let values_swap = react(input, values, swap, dt / 2.0)?;
        values = values_swap.0;
        swap = values_swap.1;

        // Full diffusion step.
        let values_swap = diffuse(input, values, swap, voxel_size_sq, dt)?;
        values = values_swap.0;
        swap = values_swap.1;
        // Potentially check for -ve values here (from sink terms).

        // Last reaction half-step.
        let values_swap = react(input, values, swap, dt / 2.0)?;
        values = values_swap.0;
        swap = values_swap.1;

        pb.tick();
    }
    pb.finish_with_message("Integration complete.");

    Ok((values, swap))
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
    let mut holder = Array2::zeros([rs, block_size]);

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
                holder[[si, n - start]] =
                    stencil.rate(input.coeffs[index], voxel_size_sq) + input.sources[index];
            }
        }

        let mut rates = rates.lock().expect("Could not lock rate array.");
        for n in start..end {
            let xi = n % rx;
            let yi = (n / rx) % ry;
            let zi = n / (rx * ry);

            for si in 0..rs {
                rates[[si, xi, yi, zi]] = holder[[si, n - start]];
            }
        }
    }
}

/// Reaction calculation function.
#[allow(clippy::expect_used)]
#[inline]
fn react(
    input: &Input,
    values: Array4<f64>,
    new_values: Array4<f64>,
    time: f64,
) -> Result<(Array4<f64>, Array4<f64>), Error> {
    debug_assert!(time > 0.0);

    let pb = ProgressBar::new("Multi-threaded", values.len() / input.specs.len());
    let pb = Arc::new(Mutex::new(pb));

    let new_values = Mutex::new(new_values);

    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let _out: Vec<_> = threads
        .par_iter()
        .map(|_id| react_impl(input, &values, &new_values, time, &Arc::clone(&pb)))
        .collect();

    let new_values = new_values
        .into_inner()
        .expect("Failed to unwrap new values array.");

    Ok((new_values, values))
}

/// Reaction calculation function.
#[allow(clippy::expect_used)]
#[inline]
fn react_impl(
    input: &Input,
    values: &Array4<f64>,
    new_values: &Mutex<Array4<f64>>,
    time: f64,
    pb: &Arc<Mutex<ProgressBar>>,
) {
    debug_assert!(time > 0.0);

    let [rs, rx, ry, _rz] = [
        input.specs.len(),
        input.grid.res()[X],
        input.grid.res()[Y],
        input.grid.res()[Z],
    ];

    let block_size = input.sett.block_size();
    let mut concs = Array2::zeros([rs, block_size]);
    let mut ks: [Array1<f64>; 4] = [
        Array1::zeros(rs),
        Array1::zeros(rs),
        Array1::zeros(rs),
        Array1::zeros(rs),
    ];

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
                concs[[si, n - start]] = values[[si, xi, yi, zi]] + MIN_POSITIVE;
            }

            let concs_ks = evolve_rk4(
                input.reactor,
                n - start,
                concs,
                ks,
                time * input.multipliers[[xi, yi, zi]],
                input.sett.quality(),
                input.sett.min_time(),
            );
            concs = concs_ks.0;
            ks = concs_ks.1;
        }

        let mut new_values = new_values.lock().expect("Could not lock value array.");
        for n in start..end {
            let xi = n % rx;
            let yi = (n / rx) % ry;
            let zi = n / (rx * ry);

            for si in 0..rs {
                new_values[[si, xi, yi, zi]] = concs[[si, n - start]] - MIN_POSITIVE;
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
    n: usize,
    mut concs: Array2<f64>,
    mut rates: [Array1<f64>; 4],
    total_time: f64,
    quality: f64,
    min_time: f64,
) -> (Array2<f64>, [Array1<f64>; 4]) {
    debug_assert!(total_time > 0.0);
    debug_assert!(quality > 0.0);
    debug_assert!(quality < 1.0);
    debug_assert!(min_time <= total_time);

    let mut cs = concs.index_axis_mut(ndarray::Axis(1), n);

    let mut time = 0.0;
    while time < total_time {
        rates[0] = reactor.deltas(&cs.view());

        let dt = ((&cs / &rates[0])
            .max()
            .expect("Failed to determine minimum rate of change.")
            * quality)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        rates[1] = reactor.deltas(&(&cs + &(&rates[0] * half_dt)).view());
        rates[2] = reactor.deltas(&(&cs + &(&rates[1] * half_dt)).view());
        rates[3] = reactor.deltas(&(&cs + &(&rates[2] * dt)).view());

        cs += &(&(&rates[0] + &(2.0 * (&rates[1] + &rates[2])) + &rates[3]) * sixth_dt);
        time += dt;
    }

    (concs, rates)
}
