//! Landscape structure.

use crate::{
    game::{Player, Tile, Viewshed},
    math::Pos2I,
};
use ndarray::Array2;
use rltk::Rltk;
use rltk::{Algorithm2D, BaseMap, Point};
use specs::{Join, World, WorldExt};

/// Landscape data
pub struct Map {
    /// Tile data.
    pub tiles: Array2<Tile>,
    /// Player spawn index.
    pub player_spawn: Pos2I,
}

impl Map {
    /// Place boundary walls.
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut tiles: Array2<Tile>, player_spawn: Pos2I) -> Self {
        let (width, height) = (tiles.nrows(), tiles.ncols());

        debug_assert!(player_spawn.x < width as i32);
        debug_assert!(player_spawn.y < height as i32);

        // Boundaries walls
        for x in 0..width {
            tiles[[x, 0]] = Tile::Wall;
            tiles[[x, height - 1]] = Tile::Wall;
        }
        for y in 0..height {
            tiles[[0, y]] = Tile::Wall;
            tiles[[width - 1, y]] = Tile::Wall;
        }

        Self {
            tiles,
            player_spawn,
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
    pub fn draw(&self, ecs: &World, ctx: &mut Rltk) {
        let mut viewsheds = ecs.write_storage::<Viewshed>();
        let mut players = ecs.write_storage::<Player>();

        let [width, height] = self.res();
        for (_player, viewshed) in (&mut players, &mut viewsheds).join() {
            for x in 0..width {
                for y in 0..height {
                    let pt = Point::new(x, y);
                    if viewshed.visible_tiles.contains(&pt) {
                        self.tiles[[x, y]].draw(ctx, x as i32, (height - y - 1) as i32);
                    }
                }
            }
        }
    }
}

impl BaseMap for Map {
    #[inline]
    #[must_use]
    fn is_opaque(&self, idx: usize) -> bool {
        let x = idx % self.width();
        let y = idx / self.width();
        self.tiles[[x, y]].is_opaque()
    }
}

impl Algorithm2D for Map {
    #[inline]
    #[must_use]
    fn dimensions(&self) -> Point {
        Point::new(self.width(), self.height())
    }
}
