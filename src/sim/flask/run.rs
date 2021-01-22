//! Simulation control functions.

use crate::{
    chem::Reactor,
    err::Error,
    sim::flask::{Input, Output},
    tools::ProgressBar,
};
use ndarray::Array1;

/// Run a Flask simulation using a single thread.
/// # Errors
/// if the progress bar can not be locked.
#[inline]
pub fn single_thread(mut concs: Array1<f64>, input: &Input) -> Result<Output, Error> {
    let steps = input.sett.dumps();

    let mut pb = ProgressBar::new("Single-threaded", steps);

    let mut time = 0.0;
    let dt = input.sett.time() / input.sett.dumps().max(1) as f64;

    for _ in 0..steps {
        concs = evolve_rk4(input.reactor, concs, dt);
        time += dt;
        pb.tick();
        println!("{:>8} : {}", time, concs);
    }

    let mut data = Output::new(input.sett.time(), input.specs, steps);
    Ok(data)
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

/// Evolve forward the given amount of time.
#[inline]
#[must_use]
fn evolve_rk4(reactor: &Reactor, mut concs: Array1<f64>, time: f64) -> Array1<f64> {
    debug_assert!(time > 0.0);

    let n = 100;
    let dt = time / n as f64;
    let half_dt = dt * 0.5;
    let sixth_dt = dt / 6.0;

    let mut k1;
    let mut k2;
    let mut k3;
    let mut k4;
    for _ in 0..n {
        k1 = reactor.deltas(&concs);
        k2 = reactor.deltas(&(&concs + &(&k1 * half_dt)));
        k3 = reactor.deltas(&(&concs + &(&k2 * half_dt)));
        k4 = reactor.deltas(&(&concs + &(&k3 * dt)));

        concs += &(&(&k1 + &(2.0 * (k2 + k3)) + &k4) * sixth_dt);
    }

    concs
}
