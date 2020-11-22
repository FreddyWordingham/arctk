//! Landscape enumeration.

/// Tile kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    /// Inaccessible.
    Wall,
    /// Open floor.
    Floor,
}
