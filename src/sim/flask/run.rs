//! Simulation control functions.

use crate::{chem::Reactor, data::Table, err::Error, sim::flask::Input, tools::ProgressBar};
use ndarray::Array1;
use ndarray_stats::QuantileExt;

/// Run a Flask simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn single_thread(mut concs: Array1<f64>, input: &Input) -> Result<Table<f64>, Error> {
    let steps = input.sett.dumps();
    let dt = input.sett.time() / (input.sett.dumps() + 1) as f64;
    let quality = input.sett.quality();
    let min_time = input.sett.min_time();

    let mut records = Vec::with_capacity(steps + 1);

    let mut pb = ProgressBar::new("Single-threaded", steps);

    for n in 0..steps {
        let mut row = Vec::with_capacity(1 + concs.len());
        row.push(dt * n as f64);
        for c in &concs {
            row.push(*c);
        }
        records.push(row);

        concs = evolve_rk4(input.reactor, concs, dt, quality, min_time);
        pb.tick();
    }

    {
        let mut row = Vec::with_capacity(1 + concs.len());
        row.push(input.sett.time());
        for c in &concs {
            row.push(*c);
        }
        records.push(row);
    }

    let mut headings = Vec::with_capacity(1 + concs.len());
    headings.push("time".to_string());
    for spec in &input.specs.names_list() {
        headings.push(spec.as_string());
    }

    Ok(Table::new(headings, records))
}

/// Evolve forward the given amount of time.
#[allow(dead_code)]
#[inline]
#[must_use]
fn evolve_euler(reactor: &Reactor, mut concs: Array1<f64>, time: f64) -> Array1<f64> {
    debug_assert!(time > 0.0);

    let n = 100;
    let dt = time / n as f64;

    for _ in 0..n {
        concs += &(&reactor.deltas(&concs) * dt);
    }

    concs
}

// /// Evolve forward the given amount of time.
// #[inline]
// #[must_use]
// fn evolve_rk4(reactor: &Reactor, mut concs: Array1<f64>, time: f64) -> Array1<f64> {
//     debug_assert!(time > 0.0);

//     let n = 100;
//     let dt = time / n as f64;
//     let half_dt = dt * 0.5;
//     let sixth_dt = dt / 6.0;

//     let mut k1;
//     let mut k2;
//     let mut k3;
//     let mut k4;
//     for _ in 0..n {
//         k1 = reactor.deltas(&concs);

//         let rs = (&concs / &k1).min().max(1e-3);

//         k2 = reactor.deltas(&(&concs + &(&k1 * half_dt)));
//         k3 = reactor.deltas(&(&concs + &(&k2 * half_dt)));
//         k4 = reactor.deltas(&(&concs + &(&k3 * dt)));

//         concs += &(&(&k1 + &(2.0 * (k2 + k3)) + &k4) * sixth_dt);
//     }

//     concs
// }

/// Evolve forward the given amount of time.
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
            .min()
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
