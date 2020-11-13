//! Ray caster implementation.

use crate::{
    geom::Ray,
    math::{rand_circle_point, rand_sphere_point, Dir3, Pos3, Vec3},
};
use arctk_attr::load;

/// Ray caster order.
#[load]
#[derive(Clone)]
pub enum Caster {
    /// Direction.
    Direction(Dir3),
    /// Target [m].
    Target(Pos3),
    /// Soft-targeting [m] [rad].
    Soft(i32, Pos3, f64),
    /// Radiant.
    Radiant(i32),
}

impl Caster {
    /// Calculate the total number of samples.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, pos: Pos3, n: i32) -> Ray {
        match *self {
            Self::Direction(dir) => Ray::new(pos, dir),
            Self::Target(tar) => Ray::new(pos, Dir3::new_normalize(tar - pos)),
            Self::Soft(samples, tar, alpha) => {
                let (r, theta) = rand_circle_point(n, samples);
                let mut ray = Ray::new(pos, Dir3::new_normalize(tar - pos));
                ray.rotate(r * alpha, theta);
                ray
            }
            Self::Radiant(samples) => {
                let (pitch, roll) = rand_sphere_point(n, samples);
                let mut ray = Ray::new(pos, Vec3::x_axis());
                ray.rotate(pitch, roll);
                ray
            }
        }
    }

    /// Retrieve the number of potential samples.
    #[inline]
    #[must_use]
    pub const fn num_samples(&self) -> i32 {
        match *self {
            Self::Direction(..) | Self::Target(..) => 1,
            Self::Soft(samples, ..) | Self::Radiant(samples) => samples,
        }
    }
}
