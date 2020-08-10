//! Diffusion thread control functions.

use crate::{
    diffusion::{Scene, Stencil},
    Bar, Vec3, X, Y,
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;

/// Run a multi-threaded diffusion simulation.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::result_expect_used)]
#[inline]
#[must_use]
pub fn multi_thread(scene: &Scene, mut concs: Array3<f64>) -> Array3<f64> {
    debug_assert!(scene.coeffs.shape() == concs.shape());

    let mut cell_size = scene.sett.boundary().widths();
    for (w, n) in cell_size.iter_mut().zip(concs.shape()) {
        *w /= *n as f64;
    }
    let dx = cell_size.min();

    let alpha = scene
        .coeffs
        .max()
        .expect("Failed to determine the maximum diffusion coefficient.");

    let max_dt = dx.powi(2) / (4.0 * alpha);

    let steps = (scene.sett.total_time() / max_dt).ceil() as u64;
    let dt = scene.sett.total_time() / steps as f64;

    let mut pb = Bar::new("Diffusing", steps);
    for _ in 0..steps {
        pb.tick();
        concs += &(diff_rate(&cell_size, &concs, scene.coeffs) * dt);
    }

    concs
}

/// Calculate the diffusion rates for each cell.
#[inline]
#[must_use]
pub fn diff_rate(cell_size: &Vec3, concs: &Array3<f64>, coeffs: &Array3<f64>) -> Array3<f64> {
    debug_assert!(concs.shape() == coeffs.shape());

    let num_cells = concs.len();

    let mut rate = Array3::zeros(concs.raw_dim());
    let res = concs.shape();

    for n in 0..num_cells {
        let xi = n % res[X];
        let yi = (n / res[X]) % res[Y];
        let zi = n / (res[X] * res[Y]);

        let index = [xi, yi, zi];

        let stencil = Stencil::new(index, concs);
        let r = stencil.rate(coeffs[index], cell_size);
        rate[index] = r;
    }

    rate
}
