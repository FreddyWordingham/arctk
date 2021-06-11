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
    path::Path,
    sync::{Arc, Mutex},
};

/// Run a multi-threaded reaction-diffusion simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn multi_thread(
    input: &Input,
    mut values: Array4<f64>,
    out_dir: &Path,
) -> Result<Array4<f64>, Error> {
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
    let max_diff_dt = min_voxel_size_sq / (8.0 * max_coeff);
    let dt = max_diff_dt * (1.0 - input.sett.d_quality()).min(1.0).max(0.0);

    let steps = 1 + input.sett.dumps();
    let step_time = input.sett.time() / steps as f64;

    // Allocation.
    let mut swap = Array4::zeros(values.raw_dim());

    // Initial value write.
    for (name, si) in input.specs.set().map() {
        values
            .index_axis(Axis(0), *si)
            .save(&out_dir.join(&format!("{:03}_{}_diff.nc", 0, name)))?;
    }

    // Main integration loop.
    // Evolution.
    let mut pb = ProgressBar::new("Diffusion-Reaction", steps);
    for n in 0..steps {
        // Evolve.
        let values_swap = evolve(input, &voxel_size_sq, step_time, dt, values, swap)?;
        values = values_swap.0;
        swap = values_swap.1;

        // Record.
        for (name, si) in input.specs.set().map() {
            values
                .index_axis(Axis(0), *si)
                .save(&out_dir.join(&format!("{:03}_{}_diff.nc", n + 1, name)))?;
        }

        // Tick.
        println!();
        pb.tick();
    }
    pb.finish_with_message("Simulation complete.");

    Ok(values)
}

/// Integrate forward a given amount of time.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn evolve(
    input: &Input,
    voxel_size_sq: &Vec3,
    time: f64,
    mut dt: f64,
    mut values: Array4<f64>,
    swap: Array4<f64>,
) -> Result<(Array4<f64>, Array4<f64>), Error> {
    debug_assert!(time > 0.0);
    debug_assert!(dt > 0.0);

    // Constants.
    let steps = 1 + (time / dt) as usize;
    dt = time / steps as f64;

    // Threading.
    let mut swap = Mutex::new(swap);

    // Evolution.
    let mut pb = ProgressBar::new("Stepping", steps);
    for _n in 0..steps {
        // First reaction half-step.
        let values_swap = react(input, dt / 2.0, values, swap)?;
        values = values_swap.0;
        swap = Mutex::new(values_swap.1);

        // Full diffusion step.
        let values_swap = diffuse(input, voxel_size_sq, dt, values, swap)?;
        values = values_swap.0;
        swap = values_swap.1;

        // Second reaction half-step.
        let values_swap = react(input, dt / 2.0, values, swap)?;
        values = values_swap.0;
        swap = Mutex::new(values_swap.1);

        // Apply source terms.
        values += &(input.sources * dt);

        // Check for zero.
        values.mapv_inplace(|x| x.max(0.0));

        // Tick.
        pb.tick();
    }

    // Unwrapping.
    let swap = swap.into_inner()?;

    Ok((values, swap))
}

/// Enact the reaction process.
/// Swap space is used to store the local updated values.
/// Note that the updated values are the first of the arrays in the pair returned.
/// # Errors
/// if the update value array could not be unwrapped from the mutex.
#[inline]
fn react(
    input: &Input,
    dt: f64,
    values: Array4<f64>,
    new_values: Mutex<Array4<f64>>,
) -> Result<(Array4<f64>, Array4<f64>), Error> {
    debug_assert!(dt > 0.0);

    let spb = Arc::new(Mutex::new(ProgressBar::new(
        "Reacting",
        values.len() / input.specs.len(),
    )));
    let num_threads = input
        .sett
        .num_threads()
        .unwrap_or(std::usize::MAX)
        .min(num_cpus::get());
    let threads: Vec<_> = (0..num_threads).collect();
    let _out: Vec<_> = threads
        .par_iter()
        .map(|_id| react_impl(input, &values, &new_values, dt, &Arc::clone(&spb)))
        .collect();

    let new_values = new_values.into_inner()?;

    Ok((new_values, values))
}

/// Enact the reactions.
#[allow(clippy::expect_used)]
#[inline]
fn react_impl(
    input: &Input,
    values: &Array4<f64>,
    new_values: &Mutex<Array4<f64>>,
    dt: f64,
    pb: &Arc<Mutex<ProgressBar>>,
) {
    debug_assert!(dt > 0.0);

    // Constants.
    let num_specs = input.specs.len();
    let res = *input.grid.res();
    let block_size = input.sett.r_block_size();
    let fraction = 1.0 - input.sett.r_quality();
    let min_time = input.sett.min_time();

    // Allocation.
    let mut holder = Array2::zeros([num_specs, block_size]);
    let mut rates: [Array1<f64>; 4] = [
        Array1::zeros(num_specs),
        Array1::zeros(num_specs),
        Array1::zeros(num_specs),
        Array1::zeros(num_specs),
    ];

    // Reaction calculation.
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        // Calculate updates.
        for n in start..end {
            let ix = crate::tools::index::linear_to_three_dim(n, &res);

            // Initialise values.
            for si in 0..num_specs {
                holder[[si, n - start]] = values[[si, ix[X], ix[Y], ix[Z]]];
            }

            let values_rates = reaction(
                n - start,
                holder,
                rates,
                input.reactor,
                dt * input.multipliers[ix],
                fraction,
                min_time,
            );
            holder = values_rates.0;
            rates = values_rates.1;
        }

        // Update values.
        {
            let mut new_values = new_values.lock().expect("Could not lock rate array.");
            for n in start..end {
                let ix = crate::tools::index::linear_to_three_dim(n, &res);
                for si in 0..num_specs {
                    let index = [si, ix[X], ix[Y], ix[Z]];
                    new_values[index] = holder[[si, n - start]];
                }
            }
            std::mem::drop(new_values);
        }
    }
}

/// Evolve forward the given amount of time using RK4.
/// Rates parameter prevents having to re-allocate for RK4 values.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn reaction(
    n: usize,
    mut values: Array2<f64>,
    mut rates: [Array1<f64>; 4],
    reactor: &Reactor,
    total_time: f64,
    fraction: f64,
    min_time: f64,
) -> (Array2<f64>, [Array1<f64>; 4]) {
    debug_assert!(total_time > 0.0);
    debug_assert!(fraction > 0.0);
    debug_assert!(fraction < 1.0);
    debug_assert!(min_time <= total_time);

    let mut vs = values.index_axis_mut(Axis(1), n);

    let mut time = 0.0;
    while time < total_time {
        // Rates and dt.
        rates[0] = reactor.deltas(&vs.view());

        let dt = (((&vs + MIN_POSITIVE) / &rates[0])
            .map(|v| v.abs())
            .min()
            .expect("Failed to determine minimum rate of change.")
            * fraction)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        rates[1] = reactor.deltas(&(&vs + &(&rates[0] * half_dt)).view());
        rates[2] = reactor.deltas(&(&vs + &(&rates[1] * half_dt)).view());
        rates[3] = reactor.deltas(&(&vs + &(&rates[2] * dt)).view());

        // Evolve values.
        vs += &(&(&rates[0] + &(2.0 * (&rates[1] + &rates[2])) + &rates[3]) * sixth_dt);

        // Progress.
        time += dt;
    }

    (values, rates)
}

/// Enact the diffusion process.
/// Swap space is used to store the local diffusion rate.
#[inline]
fn diffuse(
    input: &Input,
    voxel_size_sq: &Vec3,
    dt: f64,
    mut values: Array4<f64>,
    rates: Mutex<Array4<f64>>,
) -> Result<(Array4<f64>, Mutex<Array4<f64>>), Error> {
    debug_assert!(dt > 0.0);

    // Calculate diffusion rates.
    let spb = Arc::new(Mutex::new(ProgressBar::new(
        "Diffusing",
        values.len() / input.specs.len(),
    )));
    let threads: Vec<_> = (0..num_cpus::get()).collect();
    let _out: Vec<_> = threads
        .par_iter()
        .map(|_id| calc_diffuse_rates(input, voxel_size_sq, &values, &rates, &Arc::clone(&spb)))
        .collect();

    // Apply diffusion.
    values += &(&(*rates.lock()?) * dt);

    Ok((values, rates))
}

/// Calculate the diffusion rates.
#[allow(clippy::expect_used)]
#[inline]
fn calc_diffuse_rates(
    input: &Input,
    voxel_size_sq: &Vec3,
    values: &Array4<f64>,
    rates: &Mutex<Array4<f64>>,
    pb: &Arc<Mutex<ProgressBar>>,
) {
    // Constants.
    let num_specs = input.specs.len();
    let res = *input.grid.res();
    let block_size = input.sett.d_block_size();

    // Allocation.
    let mut holder = Array2::zeros([num_specs, block_size]);

    // Rate calculations.
    while let Some((start, end)) = {
        let mut pb = pb.lock().expect("Could not lock progress bar.");
        let b = pb.block(block_size);
        std::mem::drop(pb);
        b
    } {
        // Calculate rates.
        for n in start..end {
            let ix = crate::tools::index::linear_to_three_dim(n, &res);
            for si in 0..num_specs {
                let index = [si, ix[X], ix[Y], ix[Z]];
                let stencil = stencil::Grad::new(index, values);
                holder[[si, n - start]] = stencil.rate(input.coeffs[index], voxel_size_sq);
            }
        }

        // Store rates.
        {
            let mut rates = rates.lock().expect("Could not lock rate array.");
            for n in start..end {
                let ix = crate::tools::index::linear_to_three_dim(n, &res);
                for si in 0..num_specs {
                    let index = [si, ix[X], ix[Y], ix[Z]];
                    rates[index] = holder[[si, n - start]];
                }
            }
            std::mem::drop(rates);
        }
    }
}
