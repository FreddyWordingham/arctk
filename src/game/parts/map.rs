//! Landscape structure.

use crate::game::Tile;
use ndarray::Array2;
use rltk::Rltk;

/// Landscape data
pub struct Map {
    /// Tile data.
    pub tiles: Array2<Tile>,
}

impl Map {
    /// Place boundary walls.
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut tiles: Array2<Tile>) -> Self {
        let (width, height) = (tiles.nrows(), tiles.ncols());

        // Boundaries walls
        for x in 0..width {
            tiles[[x, 0]] = Tile::Wall;
            tiles[[x, height - 1]] = Tile::Wall;
        }
        for y in 0..height {
            tiles[[0, y]] = Tile::Wall;
            tiles[[width - 1, y]] = Tile::Wall;
        }

        Self { tiles }
    }

    /// Get the map height.
    #[inline]
    #[must_use]
    pub fn height(&self) -> usize {
        self.tiles.ncols()
    }

    /// Get the map width.
    #[inline]
    #[must_use]
    pub fn width(&self) -> usize {
        self.tiles.nrows()
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
            for y in 0..height {
                self.tiles[[x, y]].draw(ctx, x as i32, (height - y - 1) as i32);
            }
        }
    }
}
