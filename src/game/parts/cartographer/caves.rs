//! Forest map builder.

use crate::{
    game::{Cartographer, Tile, Zone},
    math::Pos2I,
};
use ndarray::Array2;
use rltk::RandomNumberGenerator;

/// Forest width.
const WIDTH: i32 = 64;
/// Forest height.
const HEIGHT: i32 = 64;
/// Maximum number of rooms to generate.
const MAX_ROOMS: usize = 40;
/// Minimum room size.
const MIN_SIZE: i32 = 6;
/// Maximum room size.
const MAX_SIZE: i32 = 10;

impl Cartographer {
    /// Construct a set of caves.
    #[inline]
    #[must_use]
    pub fn caves(rng: &mut RandomNumberGenerator) -> Self {
        // Filled map.
        let mut tiles = Array2::from_elem([WIDTH as usize, HEIGHT as usize], Tile::Wall);

        // Make the caverns.
        let mut rooms: Vec<Zone> = Vec::with_capacity(MAX_ROOMS);
        for _ in 0..MAX_ROOMS {
            let dx = rng.range(MIN_SIZE, MAX_SIZE);
            let dy = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(1, WIDTH - dx - 2);
            let y = rng.range(1, HEIGHT - dy - 2);

            let cave = Zone::new(
                Pos2I::new(x as i32, y as i32),
                Pos2I::new((x + dx) as i32, (y + dy) as i32),
            );

            if rooms.iter().all(|r| !r.intersect(&cave)) {
                let interior = cave.shrink(1);
                Self::set_zone(&mut tiles, &interior, Tile::Floor);
                rooms.push(cave);
            }
        }
        rooms.shrink_to_fit();

        // Connect them.
        for (room_a, room_b) in rooms.iter().zip(rooms.iter().skip(1)) {
            let start = room_a.center();
            let end = room_b.center();

            if rng.range(0, 2) == 0 {
                Self::set_path(&mut tiles, start, end, Tile::Floor);
            } else {
                Self::set_path_inv(&mut tiles, start, end, Tile::Floor);
            }
        }

        Self { tiles, rooms }
    }
}
