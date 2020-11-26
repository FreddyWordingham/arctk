//! Zone structure.

use crate::math::Pos2I;

/// Zone of the map.
pub struct Zone {
    /// Minimum positional bound.
    pub mins: Pos2I,
    /// Maximum positional bound.
    pub maxs: Pos2I,
}

impl Zone {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mins: Pos2I, maxs: Pos2I) -> Self {
        debug_assert!(mins < maxs);

        Self { mins, maxs }
    }

    /// Create a smaller zone.
    #[inline]
    #[must_use]
    pub fn shrink(&self, delta: i32) -> Self {
        debug_assert!(delta > 0);

        Self::new(
            Pos2I::new(self.mins.x + delta, self.mins.y + delta),
            Pos2I::new(self.maxs.x - delta, self.maxs.y - delta),
        )
    }

    /// Create a larger zone.
    #[inline]
    #[must_use]
    pub fn expand(&self, delta: i32) -> Self {
        debug_assert!(delta > 0);

        Self::new(
            Pos2I::new(self.mins.x - delta, self.mins.y - delta),
            Pos2I::new(self.maxs.x + delta, self.maxs.y + delta),
        )
    }

    /// Get the center position of the room.
    #[inline]
    #[must_use]
    pub fn center(&self) -> Pos2I {
        Pos2I::new(
            (self.mins.x + self.maxs.x) / 2,
            (self.mins.y + self.maxs.y) / 2,
        )
    }

    /// Returns true if this overlaps with another zone.
    #[inline]
    #[must_use]
    pub fn intersect(&self, rhs: &Self) -> bool {
        self.mins.x <= rhs.maxs.x
            && self.maxs.x >= rhs.mins.x
            && self.mins.y <= rhs.maxs.y
            && self.maxs.y >= rhs.mins.y
    }
}
