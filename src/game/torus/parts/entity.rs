//! Game entity structure.

use crate::{
    access,
    game::torus::{Coor2, Draw, Move2},
};

/// Game entity.
pub struct Entity {
    /// Position.
    pos: Coor2,
    /// Draw properties.
    draw: Draw,
}

impl Entity {
    access!(pos, Coor2);
    access!(draw, Draw);

    /// Construct a new entity.
    #[inline]
    #[must_use]
    pub const fn new(pos: Coor2, draw: Draw) -> Self {
        Self { pos, draw }
    }

    /// Move the entity.
    #[inline]
    pub fn travel(&mut self, delta: Move2) {
        self.pos += delta;
    }
}
