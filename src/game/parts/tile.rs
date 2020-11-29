//! Landscape enumeration.

use rltk::{Rltk, RGB};

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
    /// Draw the tile to the given position.
    #[inline]
    pub fn draw(self, ctx: &mut Rltk, x: i32, y: i32) {
        match self {
            Self::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('X'),
                );
            }
            Self::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437(' '),
                );
            }
            Self::Tree => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('^'),
                );
            }
        }
    }

    /// Check if a tile is passable.
    #[inline]
    #[must_use]
    pub fn is_passable(self) -> bool {
        match self {
            Self::Wall | Self::Tree => false,
            Self::Floor => true,
        }
    }

    /// Check if a tile is visibly blocking.
    #[inline]
    #[must_use]
    pub fn is_opaque(self) -> bool {
        match self {
            Self::Wall | Self::Tree => true,
            Self::Floor => false,
        }
    }
}
