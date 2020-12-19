//! Ray caster enumeration.

use crate::{
    fmt_report,
    geom::Ray,
    math::{rand_circle_point, rand_sphere_point, Dir3, Pos3, Vec3},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Ray caster generation.
#[file]
#[derive(Clone)]
pub enum Caster {
    /// Direction.
    Direction(Dir3),
    /// Target (m).
    Target(Pos3),
    /// Soft-targeting (samples, target (m), spread (rad)).
    Soft(i32, Pos3, f64),
    /// Radiant (samples).
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

    /// Retrieve the number of potential casts.
    #[inline]
    #[must_use]
    pub const fn num_casts(&self) -> i32 {
        match *self {
            Self::Direction(..) | Self::Target(..) => 1,
            Self::Soft(samples, ..) | Self::Radiant(samples) => samples,
        }
    }
}

impl Display for Caster {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Direction(dir) => {
                write!(fmt, "Direction: [{}, {}, {}]", dir.x, dir.y, dir.z)
            }
            Self::Target(tar) => {
                write!(fmt, "Target: [{}m, {}m, {}m]", tar.x, tar.y, tar.z)
            }
            Self::Soft(samples, tar, spread) => {
                writeln!(fmt, "Soft...")?;
                fmt_report!(fmt, samples, "samples");
                fmt_report!(
                    fmt,
                    &format!("[{}m, {}m, {}m]", tar.x, tar.y, tar.z),
                    "target"
                );
                fmt_report!(fmt, spread.to_degrees(), "spread (deg)");
                Ok(())
            }
            Self::Radiant(samples) => {
                write!(fmt, "Radiant: {}", samples)
            }
        }
    }
}
