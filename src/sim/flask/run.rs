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

    let mut pb = ProgressBar::new("Integrating", steps);
    for n in 0..steps {
        let time = dt * n as f64;
        values = evolve_rk4(values, input.sources, input.reactor, dt, quality, min_time);

        data[[n, 0]] = time;
        for (i, val) in values.iter().enumerate() {
            data[[n, i + 1]] = *val;
        }

        pb.tick();
    }

    Ok(data)
}

/// Evolve forward the given amount of time.
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
fn evolve_rk4(
    mut values: Array1<f64>,
    sources: &Array1<f64>,
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
    let mut k1;
    let mut k2;
    let mut k3;
    let mut k4;
    while time < total_time {
        k1 = reactor.deltas(&values.view());

        let dt = (((&values + MIN_POSITIVE) / &k1)
            .map(|v| v.abs())
            .min()
            .expect("Failed to determine minimum rate of change.")
            * quality)
            .max(min_time)
            .min(total_time - time);
        let half_dt = dt * 0.5;
        let sixth_dt = dt / 6.0;

        k2 = reactor.deltas(&(&values + &(&k1 * half_dt)).view());
        k3 = reactor.deltas(&(&values + &(&k2 * half_dt)).view());
        k4 = reactor.deltas(&(&values + &(&k3 * dt)).view());

        values += &(&(&k1 + &(2.0 * (k2 + k3)) + &k4) * sixth_dt);
        values += &(sources * dt);

        values.map_inplace(|elem| {
            *elem = elem.max(0.0);
        });

        time += dt;
    }

    values
}
