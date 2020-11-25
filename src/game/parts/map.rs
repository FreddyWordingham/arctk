//! Landscape structure.

use crate::{
    game::{Tile, Zone},
    ord::{X, Y},
};
use ndarray::Array2;
use rltk::{RandomNumberGenerator, Rltk};

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
    fn new(mut tiles: Array2<Tile>) -> Self {
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

    /// Construct a new forest map.
    #[inline]
    #[must_use]
    pub fn new_forest(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        // Blank map.
        let mut tiles = Array2::default(res);
        let [width, height] = res;

        // Random walls.
        let mut rng = RandomNumberGenerator::new();
        let n = tiles.len() / 8;
        for _ in 0..n {
            let x = rng.roll_dice(1, width as i32 - 2);
            let y = rng.roll_dice(1, height as i32 - 2);
            tiles[[x as usize, y as usize]] = Tile::Tree;
        }

        Self::new(tiles)
    }

    /// Construct a new caves instance.
    #[inline]
    #[must_use]
    pub fn new_caves(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        // Filled map.
        let mut tiles = Array2::from_elem(res, Tile::Wall);
        let [width, height] = res;

        let mut map = Self::new(tiles);

        // Make rooms.
        let room_a = Zone::new(20, 15, 10, 15);
        map.set_zone(&room_a, Tile::Floor);

        let room_b = Zone::new(35, 15, 10, 15);
        map.set_zone(&room_b, Tile::Floor);

        map
    }

    /// Get the tile type for a given zone.
    #[inline]
    fn set_zone(&mut self, zone: &Zone, tile: Tile) {
        for x in zone.min_x..=zone.max_x {
            for y in zone.min_y..=zone.max_y {
                self.tiles[[x as usize, y as usize]] = tile;
            }
        }
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
