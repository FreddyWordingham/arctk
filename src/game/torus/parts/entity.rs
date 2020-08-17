//! Game entity structure.

use crate::{
    access, clone,
    game::torus::{Coor2, Draw, Move2},
};
use tcod::Color;

/// Game entity.
pub struct Entity {
    /// Position.
    pos: Coor2,
    /// Symbol.
    sym: char,
    /// Colour.
    col: Color,
}

impl Entity {
    access!(pos, Coor2);
    clone!(sym, char);
    clone!(col, Color);

    /// Construct a new entity.
    #[inline]
    #[must_use]
    pub const fn new(pos: Coor2, sym: char, col: Color) -> Self {
        Self { pos, sym, col }
    }

    /// Move the entity.
    #[inline]
    pub fn travel(&mut self, delta: Move2) {
        self.pos += delta;
    }
}

impl Draw for Entity {
    #[inline]
    #[must_use]
    fn pos(&self) -> Coor2 {
        self.pos
    }

    #[inline]
    #[must_use]
    fn sym(&self) -> char {
        self.sym
    }

    #[inline]
    #[must_use]
    fn col(&self) -> Color {
        self.col
    }
}
