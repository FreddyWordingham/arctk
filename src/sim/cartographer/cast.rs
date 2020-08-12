//! Ray casting order implementation.

use crate::{cartographer::Input, sample::golden, Dir3, Group, Pos3, Ray, Trace, Vec3};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering order.
#[load]
#[derive(Clone)]
pub enum Cast {
    /// Direction.
    Direction(Dir3),
    /// Target [m].
    Target(Pos3),
    /// Soft-targeting [m] [rad].
    Soft(u64, Pos3, f64),
    /// Radiant.
    Radiant(u64),
}

impl Cast {
    /// Generate a ray from a given position.
    #[inline]
    #[must_use]
    pub fn observe_mat(&self, input: &Input, pos: Pos3) -> Option<Group> {
        match self {
            Self::Direction(dir) => Self::see_mat(input, Ray::new(pos, *dir)),
            Self::Target(tar) => {
                Self::see_mat(input, Ray::new(pos, Dir3::new_normalize(tar - pos)))
            }
            Self::Soft(samples, tar, alpha) => {
                let direct = Ray::new(pos, Dir3::new_normalize(tar - pos));
                let mut mats = Vec::with_capacity(*samples as usize);
                for n in 0..*samples {
                    let (pitch, roll) = golden::circle(n as i32, *samples as i32);
                    let mut ray = direct.clone();
                    ray.rotate(pitch * alpha, roll);
                    if let Some(mat) = Self::see_mat(input, ray) {
                        mats.push(mat);
                    }
                }
                mats.shrink_to_fit();
                crate::order::mode(&mats)
            }
            Self::Radiant(samples) => {
                let mut mats = Vec::with_capacity(*samples as usize);
                for n in 0..*samples {
                    let (pitch, roll) = golden::sphere(n as i32, *samples as i32);
                    let mut ray = Ray::new(pos, Vec3::x_axis());
                    ray.rotate(pitch, roll);
                    if let Some(mat) = Self::see_mat(input, ray) {
                        mats.push(mat);
                    }
                }
                mats.shrink_to_fit();
                crate::order::mode(&mats)
            }
        }
    }

    /// Observe the material seen with a given ray.
    #[inline]
    #[must_use]
    pub fn see_mat(input: &Input, ray: Ray) -> Option<Group> {
        let bound_dist = input
            .grid
            .boundary()
            .dist(&ray)
            .expect("Could not determine voxel distance.");

        if let Some(hit) = input.tree.observe(ray, input.sett.bump_dist(), bound_dist) {
            if let Some((inside, outside)) = input.inters.map().get(hit.group()) {
                if hit.side().is_inside() {
                    Some(inside.clone())
                } else {
                    Some(outside.clone())
                }
            } else {
                panic!("No interface entry for surface group {}", hit.group());
            }
        } else {
            None
        }
    }
}

impl Display for Cast {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Direction(dir) => format!("Direction: {:.2} {:.2} {:.2}", dir.x, dir.y, dir.z),
            Self::Target(tar) => format!("Target: {:.2} {:.2} {:.2} [m]", tar.x, tar.y, tar.z),
            Self::Soft(samples, tar, alpha) => format!(
                "Soft: {} [{:.2} {:.2} {:.2} [m]] {} [deg]",
                samples,
                tar.x,
                tar.y,
                tar.z,
                alpha.to_degrees()
            ),
            Self::Radiant(samples) => format!("Radiant: {}", samples),
        };
        write!(fmt, "{}", kind)
    }
}
