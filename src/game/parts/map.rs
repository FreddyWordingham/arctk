//! Landscape structure.

use crate::{
    game::Tile,
    ord::{X, Y},
    tools::two_dim_to_linear,
};
use ndarray::Array2;

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

        let [widht, height] = res;

        // Boundaries walls
        for x in 0..res[X] {
            tiles[[x, 0]] = Tile::Wall;
            tiles[[x, height - 1]] = Tile::Wall;
        }
        for y in 0..res[Y] {
            tiles[[0, y]] = Tile::Wall;
            tiles[[widht - 1, y]] = Tile::Wall;
        }

        Self { tiles }
    }
}
