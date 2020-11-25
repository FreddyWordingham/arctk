//! Landscape structure.

use crate::{
    game::{Tile, Zone},
    math::Pos2I,
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
        let tiles = Array2::from_elem(res, Tile::Wall);
        let [width, height] = res;
        let mut map = Self::new(tiles);

        // Settings.
        const MAX_ROOMS: usize = 60;
        const MIN_SIZE: usize = 6;
        const MAX_SIZE: usize = 10;

        let mut rng = RandomNumberGenerator::new();
        let mut rooms: Vec<Zone> = Vec::with_capacity(MAX_ROOMS);
        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.range(1, width - w - 2);
            let y = rng.range(1, height - h - 2);

            let room = Zone::new(
                Pos2I::new(x as i32, (x + w) as i32),
                Pos2I::new(y as i32, (y + h) as i32),
            );
            if rooms.iter().all(|r| !r.intersect(&room)) {
                let interior = Zone::new(
                    Pos2I::new((x + 1) as i32, (x + w - 1) as i32),
                    Pos2I::new((y + 1) as i32, (y + h - 1) as i32),
                );

                // Apply zone.
                map.set_zone(&interior, Tile::Floor);

                if !rooms.is_empty() {
                    let start = room.center();
                    let end = rooms.last().unwrap().center();
                    if rng.range(0, 2) == 1 {
                        map.set_path(start, end, Tile::Floor);
                    } else {
                        map.set_path_inv(start, end, Tile::Floor);
                    }
                }

                // Push room to list.
                rooms.push(room);
            }
        }

        map
    }

    /// Get the tile type for a given zone.
    #[inline]
    fn set_zone(&mut self, zone: &Zone, tile: Tile) {
        for x in zone.mins.x..=zone.maxs.x {
            for y in zone.mins.y..=zone.maxs.y {
                self.tiles[[x as usize, y as usize]] = tile;
            }
        }
    }

    /// Join two points with a path, using the given tile.
    #[inline]
    fn set_path(&mut self, start: Pos2I, end: Pos2I, tile: Tile) {
        let y = start.y as usize;
        for x in start.x..=end.x {
            self.tiles[[x as usize, y]] = tile;
        }
        let x = end.x as usize;
        for y in start.y..=end.y {
            self.tiles[[x, y as usize]] = tile;
        }
    }

    /// Join two points with a path, using the given tile.
    /// Inverted creation order to the standard set_path method.
    #[inline]
    fn set_path_inv(&mut self, start: Pos2I, end: Pos2I, tile: Tile) {
        let x = end.x as usize;
        for y in start.y..=end.y {
            self.tiles[[x, y as usize]] = tile;
        }
        let y = start.y as usize;
        for x in start.x..=end.x {
            self.tiles[[x as usize, y]] = tile;
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
