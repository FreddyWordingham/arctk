//! Landscape structure.

use crate::{
    game::Tile,
    ord::{X, Y},
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
        let tiles = Vec::with_capacity(res[X] * res[Y]);

        Self {
            tiles: Array2::from_shape_vec(res, tiles)
                .expect("Failed to create tile data from vec."),
        }
    }
}
