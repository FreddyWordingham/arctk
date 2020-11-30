//! Landscape enumeration.

use rltk::RGB;

/// Tile kinds
#[derive(PartialEq, Copy, Clone)]
pub enum Tile {
    /// Inaccessible.
    Wall,
    /// Open floor.
    Floor,
    /// Tree.
    Tree,
}

impl Default for Tile {
    #[inline]
    #[must_use]
    fn default() -> Self {
        Self::Floor
    }
}

impl Tile {
    /// Get the foreground colour.
    #[inline]
    #[must_use]
    pub fn fg_col(self) -> RGB {
        match self {
            Self::Wall => RGB::from_f32(0.4, 0.4, 0.6),
            Self::Floor => RGB::from_f32(0.0, 1.0, 0.0),
            Self::Tree => RGB::from_f32(0.0, 1.0, 0.0),
        }
    }

    /// Get the background colour.
    #[inline]
    #[must_use]
    pub fn bg_col(self) -> RGB {
        match self {
            Self::Wall => RGB::from_f32(0.0, 0.0, 0.0),
            Self::Floor => RGB::from_f32(0.0, 0.0, 0.0),
            Self::Tree => RGB::from_f32(0.0, 0.0, 0.0),
        }
    }

    /// Get the background colour.
    #[inline]
    #[must_use]
    pub fn char(self) -> u16 {
        rltk::to_cp437(match self {
            Self::Wall => 'X',
            Self::Floor => '.',
            Self::Tree => '^',
        })
    }

    /// Check if a tile is passable.
    #[inline]
    #[must_use]
    pub const fn is_passable(self) -> bool {
        match self {
            Self::Wall | Self::Tree => false,
            Self::Floor => true,
        }
    }

    /// Check if a tile is visibly blocking.
    #[inline]
    #[must_use]
    pub const fn is_opaque(self) -> bool {
        match self {
            Self::Wall | Self::Tree => true,
            Self::Floor => false,
        }
    }
}
