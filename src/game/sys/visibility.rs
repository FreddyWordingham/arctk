//! Visibility system

use crate::game::{Map, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};

/// Visibility system.
pub struct Visibility {}

impl Visibility {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for Visibility {
    type SystemData = (
        ReadExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
    );

    #[inline]
    fn run(&mut self, (map, mut viewshed, mut pos): Self::SystemData) {
        let w = map.width() as i32;
        let h = map.height() as i32;

        for (viewshed, pos) in (&mut viewshed, &mut pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(pos.p.x, pos.p.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < w && p.y >= 0 && p.y < h);
        }
    }
}
