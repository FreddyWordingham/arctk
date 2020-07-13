//! Orientation implementation.

use crate::{access, display_field_ln, Dir3, Pos3, Ray, Vec3};
use std::fmt::{Display, Formatter, Result};

/// Orientation structure.
#[derive(Debug)]
pub struct Orient {
    /// Position.
    pos: Pos3,
    /// Forward direction.
    forward: Dir3,
    /// Right direction.
    right: Dir3,
    /// Up direction.
    up: Dir3,
}

impl Orient {
    access!(pos, Pos3);
    access!(forward, Dir3);
    access!(right, Dir3);
    access!(up, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        let (pos, forward) = ray.destruct();
        let right = Dir3::new_normalize(forward.cross(&Vec3::z_axis()));
        let up = Dir3::new_normalize(right.cross(&forward));

        Self {
            pos,
            forward,
            up,
            right,
        }
    }

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new_tar(pos: Pos3, tar: &Pos3) -> Self {
        Self::new(Ray::new(pos, Dir3::new_normalize(tar - pos)))
    }

    /// Reference the backward direction.
    #[inline]
    #[must_use]
    pub fn back(&self) -> Dir3 {
        -self.forward
    }

    /// Reference the left direction.
    #[inline]
    #[must_use]
    pub fn left(&self) -> Dir3 {
        -self.right
    }

    /// Reference the downward direction.
    #[inline]
    #[must_use]
    pub fn down(&self) -> Dir3 {
        -self.up
    }

    /// Create a forward ray.
    #[inline]
    #[must_use]
    pub const fn forward_ray(&self) -> Ray {
        Ray::new(self.pos, self.forward)
    }

    /// Create a backward ray.
    #[inline]
    #[must_use]
    pub fn backward_ray(&self) -> Ray {
        Ray::new(self.pos, -self.forward)
    }

    /// Create a upward ray.
    #[inline]
    #[must_use]
    pub const fn up_ray(&self) -> Ray {
        Ray::new(self.pos, self.up)
    }

    /// Create a downward ray.
    #[inline]
    #[must_use]
    pub fn down_ray(&self) -> Ray {
        Ray::new(self.pos, -self.up)
    }

    /// Create a right ray.
    #[inline]
    #[must_use]
    pub const fn right_ray(&self) -> Ray {
        Ray::new(self.pos, self.right)
    }

    /// Create a left ray.
    #[inline]
    #[must_use]
    pub fn left_ray(&self) -> Ray {
        Ray::new(self.pos, -self.right)
    }
}

impl Display for Orient {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "position", self.pos, "m")?;
        display_field_ln!(
            fmt,
            "forward",
            &format!(
                "[{:.2}, {:.2}, {:.2}]",
                self.forward.x, self.forward.y, self.forward.z
            ),
            "m"
        )?;
        display_field_ln!(
            fmt,
            "right",
            &format!(
                "[{:.2}, {:.2}, {:.2}]",
                self.right.x, self.right.y, self.right.z
            ),
            "m"
        )?;
        display_field_ln!(
            fmt,
            "up",
            &format!("[{:.2}, {:.2}, {:.2}]", self.up.x, self.up.y, self.up.z),
            "m"
        )
    }
}
