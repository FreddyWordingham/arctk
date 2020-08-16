//! Game entity structure.

use crate::{
    access,
    game::torus::{Coor2, Move2},
};

/// Game entity.
pub struct Entity {
    /// Position.
    pos: Coor2,
}

impl Entity {
    access!(pos, Coor2);

    /// Construct a new entity.
    #[inline]
    #[must_use]
    pub const fn new(pos: Coor2) -> Self {
        Self { pos }
    }

    pub fn travel(&mut self, delta: Move2) {
        self.pos += delta;
    }
}
