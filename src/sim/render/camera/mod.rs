//! Camera module.

pub mod focus;
pub mod lens;
pub mod sensor;

pub use self::{focus::*, lens::*, sensor::*};

use crate::{access, display_field, display_field_ln, Ray, Rot3, X, Y};
use std::fmt::{Display, Formatter, Result};

/// Camera structure.
#[derive(Debug)]
pub struct Camera {
    /// Focus.
    focus: Focus,
    /// Lens.
    lens: Lens,
    /// Sensor.
    sensor: Sensor,
}

impl Camera {
    access!(focus, Focus);
    access!(lens, Lens);
    access!(sensor, Sensor);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(focus: Focus, lens: Lens, sensor: Sensor) -> Self {
        Self {
            focus,
            lens,
            sensor,
        }
    }

    /// Generate a new observation ray.
    #[inline]
    #[must_use]
    pub fn gen_ray(
        &self,
        pixel: [usize; 2],
        offset: f64,
        sub_sample: i32,
        depth_sample: i32,
    ) -> Ray {
        let mut ray = self.focus.observation_ray(offset, depth_sample);

        let fov = self.lens.fov();
        let delta = fov / (self.sensor.res().0 - 1) as f64;

        let mut theta = ((pixel[X] as f64) * delta) - (fov * 0.5);
        let mut phi = ((pixel[Y] as f64) * delta)
            - (fov * 0.5 * (self.sensor.res().1 as f64 / self.sensor.res().0 as f64));

        if let Some(super_sample_power) = self.sensor.super_sample_power() {
            let sub_delta = delta / f64::from(super_sample_power);
            let sx = f64::from(sub_sample % super_sample_power) + 0.5;
            let sy = f64::from(sub_sample / super_sample_power) + 0.5;
            theta += (sub_delta * (0.5 + sx)) - (delta * 0.5);
            phi += (sub_delta * (0.5 + sy)) - (delta * 0.5);
        }

        theta += self.lens().swivel()[X];
        phi += self.lens().swivel()[Y];

        *ray.dir_mut() = Rot3::from_axis_angle(&self.focus.orient().down(), theta)
            * Rot3::from_axis_angle(self.focus.orient().right(), phi)
            * ray.dir();

        ray
    }
}

impl Display for Camera {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "focus", &self.focus)?;
        display_field_ln!(fmt, "lens", &self.lens)?;
        display_field!(fmt, "sensor", &self.sensor)
    }
}
