//! Kinetic reactions control functions.

use crate::kinetics::{Reaction, Settings};
use ndarray::Array1;
use ndarray_stats::QuantileExt;

/// Run a single-threaded reaction simulation.
#[allow(clippy::module_name_repetitions)]
#[inline]
#[must_use]
pub fn single_thread(
    sett: &Settings,
    reactions: &[Reaction],
    mut concs: Array1<f64>,
    total_time: f64,
) -> Array1<f64> {
    debug_assert!(total_time > 0.0);

    let mut rates = Array1::zeros(concs.len());

    let mut time = 0.0;
    // let dt = sett.dt();
    let dt = 0.001_f64;

    while time <= total_time {
        rates.map_mut(|x| *x = 1.0e-12);
        for react in reactions {
            rates += &react.rate(&concs);
        }

        let delta = dt.min(total_time - time).max(1.0e-9);
        // println!("Delta: {}\t{}\t{}", dt, delta, time);
        concs += &(&rates * delta);
        time += delta;
    }

    concs
}
