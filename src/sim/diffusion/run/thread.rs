//! Diffusion thread control functions.

use crate::diffusion::Scene;
use ndarray::Array3;

/// Run a multi-threaded diffusion simulation.
#[inline]
#[must_use]
pub fn multi_thread(scene: &Scene, concs: Array3<f64>) -> Array3<f64> {
    concs
}
