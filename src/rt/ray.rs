//! Ray.

use nalgebra::{Point3, Rotation3, Unit, Vector3};

/// Point and direction.
#[derive(Clone)]
pub struct Ray {
    /// Origin position.
    pub pos: Point3<f64>,
    /// Direction.
    pub dir: Unit<Vector3<f64>>,
}

impl Ray {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(pos: Point3<f64>, mut dir: Unit<Vector3<f64>>) -> Self {
        dir.renormalize();
        Self { pos, dir }
    }

    /// Destruct self into positional and direction components.
    #[inline]
    #[must_use]
    pub const fn destruct(self) -> (Point3<f64>, Unit<Vector3<f64>>) {
        (self.pos, self.dir)
    }

    /// Move along the direction of travel a given distance.
    #[inline]
    pub fn travel(&mut self, dist: f64) {
        debug_assert!(dist > 0.0);

        self.pos += self.dir.as_ref() * dist;
    }

    /// Rotate the Ray with a given pitch and subsequent roll manoeuver.
    #[inline]
    pub fn rotate(&mut self, pitch: f64, roll: f64) {
        let arbitrary_axis = if (1.0 - self.dir.z.abs()) >= 1.0e-1 {
            Vector3::z_axis()
        } else {
            Vector3::y_axis()
        };

        let pitch_axis = Unit::new_normalize(self.dir.cross(&arbitrary_axis));
        let pitch_rot = Rotation3::from_axis_angle(&pitch_axis, pitch);

        let roll_rot = Rotation3::from_axis_angle(&self.dir, roll);

        self.dir = roll_rot * pitch_rot * self.dir;
        self.dir.renormalize();
    }
}
