//! Gradient-type stencil structure.

use crate::{
    fmt_report,
    math::Vec3,
    ord::{X, Y, Z},
};
use ndarray::Array4;
use std::fmt::{Display, Error, Formatter};

/// Gradient stencil implementation.
#[derive(Debug)]
pub struct Grad {
    /// Twice the central value.
    c2: f64,
    /// Previous-x value.
    prev_x: f64,
    /// Next-x value.
    next_x: f64,
    /// Previous-y value.
    prev_y: f64,
    /// Next-y value.
    next_y: f64,
    /// Previous-z value.
    prev_z: f64,
    /// Next-z value.
    next_z: f64,
}

impl Grad {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: [usize; 4], values: &Array4<f64>) -> Self {
        let shape = values.shape();
        let max_x = shape[X] - 1;
        let max_y = shape[Y] - 1;
        let max_z = shape[Z] - 1;

        let [xi, yi, zi, si] = index;

        let c2 = values[[xi, yi, zi, si]] * 2.0;

        let prev_x = if xi == 0 {
            c2 - values[[xi + 1, yi, zi, si]]
        } else {
            values[[xi - 1, yi, zi, si]]
        };
        let next_x = if xi == max_x {
            c2 - values[[xi - 1, yi, zi, si]]
        } else {
            values[[xi + 1, yi, zi, si]]
        };

        let prev_y = if yi == 0 {
            c2 - values[[xi, yi + 1, zi, si]]
        } else {
            values[[xi, yi - 1, zi, si]]
        };
        let next_y = if yi == max_y {
            c2 - values[[xi, yi - 1, zi, si]]
        } else {
            values[[xi, yi + 1, zi, si]]
        };

        let prev_z = if zi == 0 {
            c2 - values[[xi, yi, zi + 1, si]]
        } else {
            values[[xi, yi, zi - 1, si]]
        };
        let next_z = if zi == max_z {
            c2 - values[[xi, yi, zi - 1, si]]
        } else {
            values[[xi, yi, zi + 1, si]]
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
    pub fn rate(&self, coeff: f64, cell_size_sq: &Vec3) -> f64 {
        coeff
            * (((self.next_x - self.c2 + self.prev_x) / cell_size_sq.x)
                + ((self.next_y - self.c2 + self.prev_y) / cell_size_sq.y)
                + ((self.next_z - self.c2 + self.prev_z) / cell_size_sq.z))
    }
}

impl Display for Grad {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.c2, "c2");
        fmt_report!(fmt, self.prev_x, "prev x");
        fmt_report!(fmt, self.next_x, "next x");
        fmt_report!(fmt, self.prev_y, "prev y");
        fmt_report!(fmt, self.next_y, "next y");
        fmt_report!(fmt, self.prev_z, "prev z");
        fmt_report!(fmt, self.next_z, "next z");
        Ok(())
    }
}
