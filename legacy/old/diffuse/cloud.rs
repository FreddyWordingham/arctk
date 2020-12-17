//! Simulation input structure.

use super::{Data, Gradient, Settings};
use crate::{
    err::Error,
    geom::Grid,
    math::Vec3,
    ord::{X, Y},
    tools::{ProgressBar, SilentProgressBar},
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};

/// Simulation input structure.
pub struct Cloud<'a> {
    /// Coefficents.
    pub coeffs: &'a Array3<f64>,
    /// Integration settings.
    pub sett: &'a Settings,
    /// Simulation grid.
    pub grid: &'a Grid,
}

impl<'a> Cloud<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(coeffs: &'a Array3<f64>, sett: &'a Settings, grid: &'a Grid) -> Self {
        debug_assert!(coeffs.shape() == grid.res());

        Self { coeffs, sett, grid }
    }

    /// Simulate the change of given initial condition within the system.
    #[allow(clippy::expect_used)]
    #[allow(clippy::shadow_reuse)]
    #[allow(clippy::shadow_unrelated)]
    #[inline]
    #[must_use]
    pub fn sim(&self, time: f64, mut data: Data) -> Data {
        debug_assert!(time > 0.0);
        debug_assert!(data.values.shape() == self.grid.res());

        let voxel_size = self.grid.voxel_size();
        let dx = voxel_size.min();

        let max_coeff = self
            .coeffs
            .max()
            .expect("Failed to determine the maximum diffusion coefficient.");
        let max_dt = (dx * dx) / (4.0 * max_coeff);
        let dt = max_dt * self.sett.step_multiplier();

        let num_steps = (time / dt).ceil();
        let dt = time / num_steps;
        let num_steps = num_steps as u64;

        let mut rate;
        let mut pb = ProgressBar::new("Diffusing", num_steps as u64);
        for _ in 0..num_steps {
            pb.tick();

            rate = super::run::multi_thread(self, &data.values)
                .expect("Failed to calculate diffusion rate.");
            // .single_thread(&values);
            data.values += &(rate.rates * dt);
            data.time += dt;
        }

        data
    }
}
