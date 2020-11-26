//! Forest map builder.

use crate::{
    game::{Map, Tile},
    math::Pos2I,
};
use ndarray::Array2;
use rltk::RandomNumberGenerator;

/// Forest width.
const WIDTH: i32 = 64;
/// Forest height.
const HEIGHT: i32 = 64;

/// Construct a random forest.
#[inline]
#[must_use]
pub fn forest(rng: &mut RandomNumberGenerator) -> Map {
    // Blank map.
    let mut tiles = Array2::from_elem([WIDTH as usize, HEIGHT as usize], Tile::Floor);

    // Random trees.
    let n = tiles.len() / 8;
    for _ in 0..n {
        let x = rng.roll_dice(1, WIDTH as i32 - 2);
        let y = rng.roll_dice(1, HEIGHT - 2);
        tiles[[x as usize, y as usize]] = Tile::Tree;
    }

    // Spawn point.
    let player_spawn;
    loop {
        let x = rng.roll_dice(1, WIDTH as i32 - 2);
        let y = rng.roll_dice(1, HEIGHT - 2);

        if tiles[[x as usize, y as usize]].is_passable() {
            player_spawn = Pos2I::new(x, y);
            break;
        }
    }

    Map::new(tiles, player_spawn)
}
