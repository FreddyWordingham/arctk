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

            rate = self
                .multi_thread(&data.values)
                .expect("Failed to calculate diffusion rate.");
            // .single_thread(&values);
            data.values += &(rate * dt);
            data.time += dt;
        }

        data
    }

    /// Run a multi-threaded diffusion simulation.
    /// # Errors
    /// if the progress bad can not be locked.
    #[allow(clippy::expect_used)]
    #[inline]
    pub fn multi_thread(&self, values: &Array3<f64>) -> Result<Array3<f64>, Error> {
        debug_assert!(values.shape() == self.coeffs.shape());

        let pb = SilentProgressBar::new(self.grid.total_cells() as u64);
        let pb = Arc::new(Mutex::new(pb));

        let threads: Vec<_> = (0..num_cpus::get()).collect();
        let mut out: Vec<_> = threads
            .par_iter()
            .map(|_id| {
                Self::thread(
                    &Arc::clone(&pb),
                    self.sett.block_size(),
                    self.grid,
                    values,
                    self.coeffs,
                )
            })
            .collect();

        let mut data = out.pop().expect("No data received.");
        while let Some(o) = out.pop() {
            data += &o;
        }

        Ok(data)
    }

    /// Run a cartography simulation using a single thread.
    #[allow(clippy::module_name_repetitions)]
    #[inline]
    #[must_use]
    pub fn single_thread(&self, values: &Array3<f64>) -> Array3<f64> {
        let pb = SilentProgressBar::new(self.grid.total_cells() as u64);
        let pb = Arc::new(Mutex::new(pb));

        Self::thread(&pb, self.sett.block_size(), self.grid, values, self.coeffs)
    }

    /// Thread control function.
    #[allow(clippy::module_name_repetitions)]
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    fn thread(
        pb: &Arc<Mutex<SilentProgressBar>>,
        block_size: u64,
        grid: &Grid,
        values: &Array3<f64>,
        coeffs: &Array3<f64>,
    ) -> Array3<f64> {
        let rates = Array3::zeros(*grid.res());
        let voxel_size = grid.voxel_size();
        Self::diff_rate(
            pb,
            block_size,
            &Vec3::new(
                voxel_size.x * voxel_size.x,
                voxel_size.y * voxel_size.y,
                voxel_size.z * voxel_size.z,
            ),
            values,
            coeffs,
            rates,
        )
    }

    /// Calculate the diffusion rates for each cell.
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    fn diff_rate(
        pb: &Arc<Mutex<SilentProgressBar>>,
        block_size: u64,
        cell_size_sq: &Vec3,
        values: &Array3<f64>,
        coeffs: &Array3<f64>,
        mut rate: Array3<f64>,
    ) -> Array3<f64> {
        debug_assert!(values.shape() == coeffs.shape());

        let res = values.shape();
        while let Some((start, end)) = {
            let mut pb = pb.lock().expect("Could not lock progress bar.");
            let b = pb.block(block_size);
            std::mem::drop(pb);
            b
        } {
            for n in start as usize..end as usize {
                let xi = n % res[X];
                let yi = (n / res[X]) % res[Y];
                let zi = n / (res[X] * res[Y]);

                let index = [xi, yi, zi];

                let stencil = Gradient::new(index, values);
                let r = stencil.rate(coeffs[index], cell_size_sq);
                rate[index] = r;
            }
        }

        rate
    }
}
