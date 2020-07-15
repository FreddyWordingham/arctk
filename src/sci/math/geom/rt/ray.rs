//! Ray implementation.

use crate::{access, Dir3, Pos3, Rot3, Vec3};

/// Ray structure.
#[derive(Clone)]
pub struct Ray {
    /// Ray origin.
    pos: Pos3,
    /// Ray direction.
    dir: Dir3,
}

impl Ray {
    access!(pos, Pos3);
    access!(dir, dir_mut, Dir3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pos: Pos3, dir: Dir3) -> Self {
        Self { pos, dir }
    }

    /// Destruct self into components.
    #[inline]
    #[must_use]
    pub const fn destruct(self) -> (Pos3, Dir3) {
        (self.pos, self.dir)
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.pos += self.dir.as_ref() * dist;
    }

    /// Rotate the photon with a given pitch and subsequent roll manoeuvre.
    #[inline]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let arbitrary_axis = if (self.dir.z.abs() - 1.0) >= 1.0e-1 {
            Vec3::y_axis()
        } else {
            Vec3::z_axis()
        };

        let pitch_axis = Dir3::new_normalize(self.dir.cross(&arbitrary_axis));
        let pitch_rot = Rot3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rot3::from_axis_angle(&self.dir, roll);

        self.dir = roll_rot * pitch_rot * self.dir;
        self.dir.renormalize();
    }
}
