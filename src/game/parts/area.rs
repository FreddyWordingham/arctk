//! Area structure.

/// Area of the map.
pub struct Area {
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32,
}

impl Area {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> Area {
        debug_assert!(min_x < max_x);
        debug_assert!(min_y < max_y);

        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// Returns true if this overlaps with another area.
    #[inline]
    #[must_use]
    pub fn intersect(&self, rhs: &Area) -> bool {
        self.min_x <= rhs.max_x
            && self.max_x >= rhs.min_x
            && self.min_y <= rhs.max_y
            && self.max_y >= rhs.min_y
    }

    /// Get the center position of the room.
    #[inline]
    #[must_use]
    pub fn center(&self) -> (i32, i32) {
        ((self.min_x + self.max_x) / 2, (self.min_y + self.max_y) / 2)
    }
}
