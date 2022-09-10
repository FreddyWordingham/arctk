//! Orientation.

use nalgebra::{Point3, Unit, Vector3};

use crate::rt::Ray;

/// Orientation.
pub struct Orientation {
    /// Position.
    pub pos: Point3<f64>,
    /// Forward direction.
    pub forward: Unit<Vector3<f64>>,
    /// Right direction.
    pub right: Unit<Vector3<f64>>,
    /// Up direction.
    pub up: Unit<Vector3<f64>>,
}

impl Orientation {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(ray: Ray) -> Self {
        let (pos, forward) = ray.destruct();
        let right = if forward.z.abs() <= 0.9 {
            Unit::new_normalize(forward.cross(&Vector3::z_axis())) // Universal up is z-axis.
        } else {
            Unit::new_normalize(forward.cross(&Vector3::x_axis())) // If facing along z-axis, compute relative up using x-axis.
        };
        let up = Unit::new_normalize(right.cross(&forward));

        Self {
            pos,
            forward,
            right,
            up,
        }
    }

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new_tar(pos: Point3<f64>, tar: &Point3<f64>) -> Self {
        Self::new(Ray::new(pos, Unit::new_normalize(tar - pos)))
    }

    /// Reference the backward direction.
    #[inline]
    #[must_use]
    pub fn back(&self) -> Unit<Vector3<f64>> {
        -self.forward
    }

    /// Reference the left direction.
    #[inline]
    #[must_use]
    pub fn left(&self) -> Unit<Vector3<f64>> {
        -self.right
    }

    /// Reference the downward direction.
    #[inline]
    #[must_use]
    pub fn down(&self) -> Unit<Vector3<f64>> {
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
