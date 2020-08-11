//! Diffusion thread control functions.

use crate::{
    diffusion::{Gradient, Settings},
    Bar, Vec3, X, Y,
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;

/// Run a single-threaded diffusion simulation.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::expect_used)]
#[inline]
#[must_use]
pub fn single_thread(sett: &Settings, coeffs: &Array3<f64>, mut concs: Array3<f64>) -> Array3<f64> {
    debug_assert!(coeffs.shape() == concs.shape());

    let mut cell_size = sett.boundary().widths();
    for (w, n) in cell_size.iter_mut().zip(concs.shape()) {
        *w /= *n as f64;
    }
    let dx = cell_size.min();

    let max_coeff = coeffs
        .max()
        .expect("Failed to determine the maximum diffusion coefficient.");
    let max_dt = dx.powi(2) / (4.0 * max_coeff);

    let time_delta = sett.total_time() / (sett.num_dumps() + 1) as f64;
    let dt = (max_dt * sett.step_frac()).min(time_delta);
    let steps = (time_delta / dt).ceil() as u64;
    let dt = time_delta / steps as f64;

    let mut pb = Bar::new("Diffusing", steps);
    for _ in 0..steps {
        pb.tick();
        concs += &(diff_rate(&cell_size, &concs, coeffs) * dt);
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

        let stencil = Gradient::new(index, concs);
        let r = stencil.rate(coeffs[index], cell_size);
        rate[index] = r;
    }

    rate
}
