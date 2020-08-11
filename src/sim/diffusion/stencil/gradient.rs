//! Gradient-type stencil structure.

use crate::{X, Y, Z};
use nalgebra::Vector3;
use ndarray::Array3;

/// Gradient stencil implementation.
#[derive(Debug)]
pub struct Gradient {
    /// Twice the central concentration.
    c2: f64,
    /// Previous-x concentration.
    prev_x: f64,
    /// Next-x concentration.
    next_x: f64,
    /// Previous-y concentration.
    prev_y: f64,
    /// Next-y concentration.
    next_y: f64,
    /// Previous-z concentration.
    prev_z: f64,
    /// Next-z concentration.
    next_z: f64,
}

impl Gradient {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: [usize; 3], concs: &Array3<f64>) -> Self {
        let maxs = concs.shape();
        let max_x = maxs[X] - 1;
        let max_y = maxs[Y] - 1;
        let max_z = maxs[Z] - 1;

        let [xi, yi, zi] = index;

        let c2 = concs[[xi, yi, zi]] * 2.0;

        let prev_x = if xi == 0 {
            c2 - concs[[xi + 1, yi, zi]]
        } else {
            concs[[xi - 1, yi, zi]]
        };
        let next_x = if xi == max_x {
            c2 - concs[[xi - 1, yi, zi]]
        } else {
            concs[[xi + 1, yi, zi]]
        };

        let prev_y = if yi == 0 {
            c2 - concs[[xi, yi + 1, zi]]
        } else {
            concs[[xi, yi - 1, zi]]
        };
        let next_y = if yi == max_y {
            c2 - concs[[xi, yi - 1, zi]]
        } else {
            concs[[xi, yi + 1, zi]]
        };

        let prev_z = if zi == 0 {
            c2 - concs[[xi, yi, zi + 1]]
        } else {
            concs[[xi, yi, zi - 1]]
        };
        let next_z = if zi == max_z {
            c2 - concs[[xi, yi, zi - 1]]
        } else {
            concs[[xi, yi, zi + 1]]
        };

        Self {
            c2,
            prev_x,
            next_x,
            prev_y,
            next_y,
            prev_z,
            next_z,
        }
    }

    /// Calculate the rate of diffusion.
    #[inline]
    #[must_use]
    pub fn rate(&self, coeff: f64, cell_size: &Vector3<f64>) -> f64 {
        coeff
            * (((self.next_x - self.c2 + self.prev_x) / cell_size.x.powi(2))
                + ((self.next_y - self.c2 + self.prev_y) / cell_size.y.powi(2))
                + ((self.next_z - self.c2 + self.prev_z) / cell_size.z.powi(2)))
    }
}
