//! Forest map builder.

use crate::game::{Cartographer, Tile};
use ndarray::Array2;
use rltk::RandomNumberGenerator;

/// Forest width.
const WIDTH: i32 = 64;
/// Forest height.
const HEIGHT: i32 = 64;

impl Cartographer {
    /// Construct a random forest.
    #[inline]
    #[must_use]
    pub fn forest(rng: &mut RandomNumberGenerator) -> Self {
        // Blank map.
        let mut tiles = Array2::from_elem([WIDTH as usize, HEIGHT as usize], Tile::Floor);

        // Random trees.
        let n = tiles.len() / 8;
        for _ in 0..n {
            let x = rng.roll_dice(1, WIDTH as i32 - 2);
            let y = rng.roll_dice(1, HEIGHT - 2);
            tiles[[x as usize, y as usize]] = Tile::Tree;
        }

        Self {
            tiles,
            rooms: vec![],
        }
    }
}
