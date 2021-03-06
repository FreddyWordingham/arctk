//! Simulation control functions.

use crate::{chem::Reactor, err::Error, sim::flask::Input, tools::ProgressBar};
use ndarray::{Array1, Array2};
use ndarray_stats::QuantileExt;
use std::f64::MIN_POSITIVE;

/// Run a Flask simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn run(mut values: Array1<f64>, input: &Input) -> Result<Array2<f64>, Error> {
    // Constants.
    let steps = input.sett.dumps() + 1;
    let dt = input.sett.time() / (input.sett.dumps() + 1) as f64;
    let fraction = 1.0 - input.sett.quality();
    let min_time = input.sett.min_time();

    // Allocation.
    let mut data = Array2::zeros([steps + 1, values.len() + 1]);
    let mut rates = [
        Array1::zeros(values.len()),
        Array1::zeros(values.len()),
        Array1::zeros(values.len()),
        Array1::zeros(values.len()),
    ];

    // Initial value write.
    data[[0, 0]] = 0.0;
    for (i, val) in values.iter().enumerate() {
        data[[0, i + 1]] = *val;
    }

    // Main integration loop.
    let mut pb = ProgressBar::new("Reacting", steps);
    for n in 0..steps {
        // React.
        let time = dt * (n + 1) as f64;
        values = react(
            values,
            input.sources,
            &mut rates,
            input.reactor,
            dt,
            fraction,
            min_time,
        );

        // Record.
        data[[n + 1, 0]] = time;
        for (i, val) in values.iter().enumerate() {
            data[[n + 1, i + 1]] = *val;
        }

        // Tick.
        pb.tick();
    }
    pb.finish_with_message("Reaction complete.");

    Ok(data)
}

/// Evolve forward the given amount of time using RK4.
/// Rates parameter prevents having to re-allocate for RK4 values.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn react(
    mut values: Array1<f64>,
    sources: &Array1<f64>,
    rates: &mut [Array1<f64>; 4],
    reactor: &Reactor,
    total_time: f64,
    fraction: f64,
    min_time: f64,
) -> Array1<f64> {
    debug_assert!(total_time > 0.0);
    debug_assert!(fraction > 0.0);
    debug_assert!(fraction < 1.0);
    debug_assert!(min_time <= total_time);

    let mut time = 0.0;
    while time < total_time {
        // Rates and dt.
        rates[0] = reactor.deltas(&values.view());

        let dt = (((&values + MIN_POSITIVE) / &rates[0])
            .map(|v| v.abs())
            .min()
            .expect("Failed to determine minimum rate of change.")
            * fraction)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        rates[1] = reactor.deltas(&(&values + &(&rates[0] * half_dt)).view());
        rates[2] = reactor.deltas(&(&values + &(&rates[1] * half_dt)).view());
        rates[3] = reactor.deltas(&(&values + &(&rates[2] * dt)).view());

        // Evolve values.
        values += &(&(&rates[0] + &(2.0 * (&rates[1] + &rates[2])) + &rates[3]) * sixth_dt);

        // Apply source terms.
        values += &(sources * dt);

        // Check for zero.
        values.mapv_inplace(|x| x.max(0.0));

        // Progress.
        time += dt;
    }

    values
}
