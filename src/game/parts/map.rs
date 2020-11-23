//! Landscape structure.

use crate::{
    game::Tile,
    ord::{X, Y},
};
use ndarray::Array2;
use rltk::Rltk;

/// Landscape data
pub struct Map {
    /// Tile data.
    tiles: Array2<Tile>,
}

impl Map {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        let mut tiles = Array2::default(res);

        let [width, height] = res;

        // Boundaries walls
        for x in 0..res[X] {
            tiles[[x, 0]] = Tile::Wall;
            tiles[[x, height - 1]] = Tile::Wall;
        }
        for y in 0..res[Y] {
            tiles[[0, y]] = Tile::Wall;
            tiles[[width - 1, y]] = Tile::Wall;
        }

        Self { tiles }
    }

    /// Get the map height.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.tiles.nrows()
    }

    /// Get the map width.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.tiles.ncols()
    }

    /// Get the map resolution.
    #[inline]
    #[must_use]
    pub fn res(&self) -> [usize; 2] {
        [self.width(), self.height()]
    }

    /// Draw the map.
    #[inline]
    pub fn draw(&self, ctx: &mut Rltk) {
        let [width, height] = self.res();
        for x in 0..width {
            for y in 0..width {
                self.tiles[[x, y]].draw(ctx, x as i32, y as i32);
            }
        }
    }
}
