//! Kinetic reactions control functions.

use crate::kinetics::{Reaction, Settings};
use ndarray::Array1;

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

    let mut deltas = Array1::zeros(concs.len());

    let mut time = 0.0;
    let dt = sett.dt();
    while time <= total_time {
        deltas.map_mut(|x| *x = 0.0);
        for react in reactions {
            deltas += &react.rate(&concs);
            concs += &(&deltas * dt);
        }
        time += dt;
    }

    concs
}
