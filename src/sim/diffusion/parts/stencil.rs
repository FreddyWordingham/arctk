//! Stencil-type enumeration.

use nalgebra::Vector3;
use ndarray::Array3;

/// Stencil enumeration implementation.
#[derive(Debug)]
pub struct Stencil {
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

impl Stencil {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: [usize; 3], concs: &Array3<f64>) -> Self {
        let c2 = 1.0;
        let prev_x = 1.0;
        let next_x = 1.0;
        let prev_y = 1.0;
        let next_y = 1.0;
        let prev_z = 1.0;
        let next_z = 1.0;

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
