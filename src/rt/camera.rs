//! Observation camera.

use nalgebra::Rotation3;

use crate::rt::{Orientation, Ray};

/// Ray emitter object.
pub struct Camera {
    /// Orientation.
    pub orient: Orientation,
    /// Resolution.
    pub res: [usize; 2],
    /// Super sampling power.
    pub ss_power: usize,
    /// Rotation delta.
    half_delta_theta: f64,
}

impl Camera {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(orient: Orientation, fov: f64, res: [usize; 2], ss_power: usize) -> Self {
        debug_assert!(fov > 0.0);
        debug_assert!(res[0] > 0);
        debug_assert!(res[1] > 0);
        debug_assert!(ss_power > 0);

        let half_delta_theta = fov / ((2 * (ss_power * (res[0] - 1))) as f64);

        Self {
            orient,
            res,
            ss_power,
            half_delta_theta,
        }
    }

    /// Emit a ray for the given pixel and super-sample.
    #[inline]
    #[must_use]
    pub fn emit(&self, pixel: [usize; 2], ss: [usize; 2]) -> Ray {
        debug_assert!(pixel[0] < self.res[0]);
        debug_assert!(pixel[1] < self.res[1]);
        debug_assert!(ss[0] < self.ss_power);
        debug_assert!(ss[1] < self.ss_power);

        let mut theta =
            self.half_delta_theta * (1 + (2 * (ss[0] + (pixel[0] * self.ss_power)))) as f64;
        let mut phi =
            self.half_delta_theta * (1 + (2 * (ss[1] + (pixel[1] * self.ss_power)))) as f64;

        theta -= self.half_delta_theta * (self.res[0] * self.ss_power) as f64;
        phi -= self.half_delta_theta * (self.res[1] * self.ss_power) as f64;

        let mut ray = self.orient.forward_ray();
        ray.dir = Rotation3::from_axis_angle(&self.orient.down(), theta)
            * Rotation3::from_axis_angle(&self.orient.right, phi)
            * ray.dir;

        ray
    }
}
