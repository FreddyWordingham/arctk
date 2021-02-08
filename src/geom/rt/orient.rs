//! Orientation implementation.

use crate::{
    access, fmt_report,
    geom::Ray,
    math::{Dir3, Pos3, Vec3},
};
use std::fmt::{Display, Error, Formatter};

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
    access!(pos, pos_mut, Pos3);
    access!(forward, Dir3);
    access!(right, Dir3);
    access!(up, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        let (pos, forward) = ray.destruct();
        let right = if forward.z.abs() <= 0.9 {
            Dir3::new_normalize(forward.cross(&Vec3::z_axis())) // Universal up is z-axis.
        } else {
            Dir3::new_normalize(forward.cross(&Vec3::x_axis())) // If facing along z-axis, compute relative up using x-axis.
        };
        let up = Dir3::new_normalize(right.cross(&forward));

        Self {
            pos,
            forward,
            up,
            right,
        }
    }

    /// Construct with an up-direction.
    #[inline]
    #[must_use]
    pub fn new_up(ray: Ray, up: &Dir3) -> Self {
        let (pos, forward) = ray.destruct();
        let right = Dir3::new_normalize(forward.cross(up));
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
    pub fn forward_ray(&self) -> Ray {
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
    pub fn up_ray(&self) -> Ray {
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
    pub fn right_ray(&self) -> Ray {
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
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!("({}, {}, {})", self.pos.x, self.pos.y, self.pos.z),
            "position (m)"
        );
        fmt_report!(
            fmt,
            &format!(
                "({}, {}, {})",
                self.forward.x, self.forward.y, self.forward.z
            ),
            "forwards"
        );
        fmt_report!(
            fmt,
            &format!("({}, {}, {})", self.right.x, self.right.y, self.right.z),
            "rightwards"
        );
        fmt_report!(
            fmt,
            &format!("({}, {}, {})", self.up.x, self.up.y, self.up.z),
            "upwards"
        );
        Ok(())
    }
}
