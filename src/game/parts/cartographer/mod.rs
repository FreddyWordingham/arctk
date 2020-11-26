//! Map producing structure.

use crate::{
    game::{Map, Tile, Zone},
    math::Pos2I,
};
use ndarray::Array2;

pub mod caves;
pub mod forest;

pub use self::caves::*;
pub use self::forest::*;

/// Map builder.
pub struct Cartographer {
    /// Tile data.
    pub tiles: Array2<Tile>,
    /// Rooms.
    pub rooms: Vec<Zone>,
}

impl Cartographer {
    /// Build the map instance.
    #[inline]
    #[must_use]
    pub fn build(self) -> Map {
        Map::new(self.tiles)
    }

    /// Get the tile type for a given zone.
    #[inline]
    fn set_zone(tiles: &mut Array2<Tile>, zone: &Zone, tile: Tile) {
        for x in zone.mins.x..=zone.maxs.x {
            for y in zone.mins.y..=zone.maxs.y {
                tiles[[x as usize, y as usize]] = tile;
            }
        }
    }

    /// Join two points with a path, using the given tile.
    #[inline]
    fn set_path(tiles: &mut Array2<Tile>, start: &Pos2I, end: &Pos2I, tile: Tile) {
        let y = start.y as usize;
        let x0 = start.x.min(end.x);
        let x1 = start.x.max(end.x);
        for x in x0..=x1 {
            tiles[[x as usize, y]] = tile;
        }

        let x = end.x as usize;
        let y0 = start.y.min(end.y);
        let y1 = start.y.max(end.y);
        for y in y0..=y1 {
            tiles[[x, y as usize]] = tile;
        }
    }

    /// Join two points with a path, using the given tile.
    /// Inverted creation order to the standard set_path method.
    #[inline]
    fn set_path_inv(tiles: &mut Array2<Tile>, start: &Pos2I, end: &Pos2I, tile: Tile) {
        let x = end.x as usize;
        let y0 = start.y.min(end.y);
        let y1 = start.y.max(end.y);
        for y in y0..=y1 {
            tiles[[x, y as usize]] = tile;
        }

        let y = start.y as usize;
        let x0 = start.x.min(end.x);
        let x1 = start.x.max(end.x);
        for x in x0..=x1 {
            tiles[[x as usize, y]] = tile;
        }
    }
}
