//! Camera structure.

use crate::{
    access,
    geom::{Orient, Ray},
    ord::{X, Y},
};

/// Tracer emission structure.
pub struct Camera {
    /// Orientation.
    orient: Orient,
    /// Horizontal field-of-view [rad].
    fov: f64,
    /// Image resolution.
    res: [usize; 2],
    /// Super-sampling power.
    super_sampling: u64,
}

impl Camera {
    access!(res, [usize; 2]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(orient: Orient, fov: f64, res: [usize; 2], super_sampling: u64) -> Self {
        debug_assert!(fov > 0.0);
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(super_sampling > 0);

        Self {
            orient,
            fov,
            res,
            super_sampling,
        }
    }

    /// Calculate the total number of pixels.
    #[inline]
    #[must_use]
    pub const fn num_pixels(&self) -> usize {
        self.res[X] * self.res[Y]
    }

    /// Calculate the number of super-samples per pixel.
    #[inline]
    #[must_use]
    pub const fn super_samples(&self) -> u64 {
        self.super_sampling * self.super_sampling
    }

    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub fn num_samples(&self) -> u64 {
        self.super_samples() * self.num_pixels() as u64
    }

    /// Emit the nth ray.
    #[inline]
    #[must_use]
    pub fn emit(&self, n: u64) -> Ray {
        self.orient.forward_ray()
    }
}
