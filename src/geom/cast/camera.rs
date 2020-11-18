//! Camera structure.

use crate::{
    access, clone,
    geom::{Orient, Ray},
    math::{Pos3, Rot3},
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
    ss_power: usize,
}

impl Camera {
    access!(res, [usize; 2]);
    clone!(ss_power, usize);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(orient: Orient, fov: f64, res: [usize; 2], ss_power: usize) -> Self {
        debug_assert!(fov > 0.0);
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(ss_power > 0);

        let half_delta_theta = fov / ((2 * (ss_power * (res[X] - 1))) as f64);

        Self {
            orient,
            half_delta_theta,
            res,
            ss_power,
        }
    }

    /// Reference the camera's position.
    #[inline]
    #[must_use]
    pub fn pos(&self) -> &Pos3 {
        self.orient.pos()
    }

    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub const fn num_pixels(&self) -> usize {
        self.res[X] * self.res[Y]
    }

    /// Calculate the total number of super samples per pixel.
    #[inline]
    #[must_use]
    pub const fn num_super_samples(&self) -> usize {
        self.ss_power * self.ss_power
    }

    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub const fn num_samples(&self) -> usize {
        self.num_super_samples() * self.num_pixels() as usize
    }

    /// Emit a ray for the given pixel and super-sample.
    #[inline]
    #[must_use]
    pub fn emit(&self, pixel: [usize; 2], ss: [usize; 2]) -> Ray {
        debug_assert!(pixel[X] < self.res[X]);
        debug_assert!(pixel[Y] < self.res[Y]);
        debug_assert!(ss[X] < self.ss_power);
        debug_assert!(ss[Y] < self.ss_power);

        let mut theta =
            self.half_delta_theta * (1 + (2 * (ss[X] + (pixel[X] * self.ss_power)))) as f64;
        let mut phi =
            self.half_delta_theta * (1 + (2 * (ss[Y] + (pixel[Y] * self.ss_power)))) as f64;

        theta -= self.half_delta_theta * (self.res[X] * self.ss_power) as f64;
        phi -= self.half_delta_theta * (self.res[Y] * self.ss_power) as f64;

        let mut ray = self.orient.forward_ray();
        *ray.dir_mut() = Rot3::from_axis_angle(&self.orient.down(), theta)
            * Rot3::from_axis_angle(self.orient.right(), phi)
            * ray.dir();

        ray
    }
}
