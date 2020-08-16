//! Drawing information structure.

use crate::clone;
use tcod::Color;

/// Drawing information.
pub struct Draw {
    /// Symbol.
    sym: char,
    /// Colour.
    col: Color,
}

impl Draw {
    clone!(sym, char);
    clone!(col, Color);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sym: char, col: Color) -> Self {
        Self { sym, col }
    }
}
