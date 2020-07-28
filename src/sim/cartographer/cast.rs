//! Ray casting order implementation.

use crate::{Dir3, Pos3, Ray};
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
}

impl Cast {
    /// Generate a ray from a given position.
    #[inline]
    #[must_use]
    pub fn gen_ray(&self, pos: Pos3) -> Ray {
        match self {
            Self::Direction(dir) => Ray::new(pos, *dir),
            Self::Target(tar) => Ray::new(pos, Dir3::new_normalize(tar - pos)),
        }
    }
}

impl Display for Cast {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Direction(dir) => format!("Direction: {:.2} {:.2} {:.2}", dir.x, dir.y, dir.z),
            Self::Target(tar) => format!("Target: {:.2} {:.2} {:.2} [m]", tar.x, tar.y, tar.z),
        };
        write!(fmt, "{}", kind)
    }
}
