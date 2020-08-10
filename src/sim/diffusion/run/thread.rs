//! Diffusion thread control functions.

use crate::{diffusion::Scene, order};
use ndarray::Array3;
use ndarray_stats::QuantileExt;

/// Run a multi-threaded diffusion simulation.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::result_expect_used)]
#[inline]
#[must_use]
pub fn multi_thread(scene: &Scene, concs: Array3<f64>) -> Array3<f64> {
    let dx = order::min(
        &scene
            .sett
            .boundary()
            .widths()
            .iter()
            .zip(concs.shape())
            .map(|(w, r)| w / *r as f64)
            .collect::<Vec<f64>>(),
    );
    let alpha = scene
        .coeffs
        .max()
        .expect("Failed to determine the maximum diffusion coefficient.");

    let max_dt = dx.powi(2) / (4.0 * alpha);

    concs
}
