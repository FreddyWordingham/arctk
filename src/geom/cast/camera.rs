//! Camera structure.

use crate::{
    access,
    geom::{Orient, Ray},
    math::Rot3,
    ord::{X, Y},
};

/// Tracer emission structure.
pub struct Camera {
    /// Orientation.
    orient: Orient,
    /// Rotation delta.
    half_delta_theta: f64,
    /// Resolution.
    res: [usize; 2],
    /// Super sampling power.
    super_sampling: usize,
}

impl Camera {
    access!(res, [usize; 2]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(orient: Orient, fov: f64, res: [usize; 2], super_sampling: usize) -> Self {
        debug_assert!(fov > 0.0);
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(super_sampling > 0);

        let delta_theta = fov / ((super_sampling as usize * res[X]) as f64);

        Self {
            orient,
            half_delta_theta: delta_theta / 2.0,
            res,
            super_sampling,
        }
    }

    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub fn num_pixels(&self) -> usize {
        self.res[X] * self.res[Y]
    }

    /// Calculate the total number of super samples per pixel.
    #[inline]
    #[must_use]
    pub fn num_super_samples(&self) -> usize {
        self.super_sampling * self.super_sampling
    }

    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub fn num_samples(&self) -> usize {
        self.num_super_samples() * self.num_pixels() as usize
    }

    /// Emit a ray for the given pixel and super-sample.
    #[inline]
    #[must_use]
    pub fn emit(&self, pixel: [usize; 2], ss: [usize; 2]) -> Ray {
        debug_assert!(pixel[X] < self.res[X]);
        debug_assert!(pixel[Y] < self.res[Y]);
        debug_assert!(ss[X] < self.super_sampling);
        debug_assert!(ss[Y] < self.super_sampling);

        let theta = self.half_delta_theta
            * (1.0 + (ss[X] as f64 * (pixel[X] as f64 + 1.0 - self.res[X] as f64)));
        let phi = self.half_delta_theta
            * (1.0 + (ss[Y] as f64 * (pixel[Y] as f64 + 1.0 - self.res[Y] as f64)));

        let mut ray = self.orient.forward_ray();
        *ray.dir_mut() = Rot3::from_axis_angle(&self.orient.down(), theta)
            * Rot3::from_axis_angle(self.orient.right(), phi)
            * ray.dir();

        ray
    }
}
