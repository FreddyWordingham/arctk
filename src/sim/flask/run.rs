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
    let steps = input.sett.dumps() + 1;
    let dt = input.sett.time() / (input.sett.dumps() + 1) as f64;
    let quality = 1.0 - input.sett.quality();
    let min_time = input.sett.min_time();

    let mut data = Array2::zeros([steps + 1, values.len() + 1]);
    let mut rates = [
        Array1::zeros(values.len()),
        Array1::zeros(values.len()),
        Array1::zeros(values.len()),
        Array1::zeros(values.len()),
    ];

    data[[0, 0]] = 0.0;
    for (i, val) in values.iter().enumerate() {
        data[[0, i + 1]] = *val;
    }

    let mut pb = ProgressBar::new("Reacting", steps);
    for n in 0..steps {
        let time = dt * (n + 1) as f64;
        values = evolve_rk4(
            values,
            input.sources,
            &mut rates,
            input.reactor,
            dt,
            quality,
            min_time,
        );

        data[[n + 1, 0]] = time;
        for (i, val) in values.iter().enumerate() {
            data[[n + 1, i + 1]] = *val;
        }

        pb.tick();
    }
    pb.finish_with_message("Integration complete.");

    Ok(data)
}

/// Evolve forward the given amount of time using RK4.
/// Rates parameter prevents having to re-allocate for RK4 values.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn evolve_rk4(
    mut values: Array1<f64>,
    sources: &Array1<f64>,
    rates: &mut [Array1<f64>; 4],
    reactor: &Reactor,
    total_time: f64,
    quality: f64,
    min_time: f64,
) -> Array1<f64> {
    debug_assert!(total_time > 0.0);
    debug_assert!(quality > 0.0);
    debug_assert!(quality < 1.0);
    debug_assert!(min_time <= total_time);

    let mut time = 0.0;
    while time < total_time {
        rates[0] = reactor.deltas(&values.view());

        let dt = (((&values + MIN_POSITIVE) / &rates[0])
            .map(|v| v.abs())
            .min()
            .expect("Failed to determine minimum rate of change.")
            * quality)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        rates[1] = reactor.deltas(&(&values + &(&rates[0] * half_dt)).view());
        rates[2] = reactor.deltas(&(&values + &(&rates[1] * half_dt)).view());
        rates[3] = reactor.deltas(&(&values + &(&rates[2] * dt)).view());

        values += &(&(&rates[0] + &(2.0 * (&rates[1] + &rates[2])) + &rates[3]) * sixth_dt);
        values += &(sources * dt);

        values.map_inplace(|elem| {
            *elem = elem.max(0.0);
        });

        time += dt;
    }

    values
}
