//! Landscape enumeration.

/// Tile kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    /// Inaccessible.
    Wall,
    /// Open floor.
    Floor,
}

impl Default for Tile {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::Floor
    }
}
