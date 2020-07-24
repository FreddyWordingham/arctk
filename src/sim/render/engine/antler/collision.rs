//! Collision structure.

use crate::Dir3;

/// Collision event return information.
pub struct Collision {
    /// Model index.
    pub index: usize,
    /// Surface normal.
    pub norm: Dir3,
}

impl Collision {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(index: usize, norm: Dir3) -> Self {
        Self { index, norm }
    }
}
