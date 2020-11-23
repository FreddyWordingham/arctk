//! Landscape enumeration.

use rltk::{Rltk, RGB};

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

impl Tile {
    /// Draw the tile to the given position.
    #[inline]
    pub fn draw(&self, ctx: &mut Rltk, x: i32, y: i32) {
        match self {
            Self::Wall => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.5, 0.5, 0.5),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('.'),
                );
            }
            Self::Floor => {
                ctx.set(
                    x,
                    y,
                    RGB::from_f32(0.0, 1.0, 0.0),
                    RGB::from_f32(0.0, 0.0, 0.0),
                    rltk::to_cp437('#'),
                );
            }
        }
    }
}
