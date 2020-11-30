//! Visibility system

use crate::game::{Map, Player, Position, Viewshed};
use rltk::{field_of_view, Point};
use specs::{Entities, Join, ReadStorage, System, WriteExpect, WriteStorage};

/// Visibility system.
pub struct Visibility {}

impl Visibility {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl<'a> System<'a> for Visibility {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    #[inline]
    fn run(&mut self, (mut map, entities, mut viewshed, mut pos, player): Self::SystemData) {
        let w = map.width() as i32;
        let h = map.height() as i32;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &mut pos).join() {
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles =
                field_of_view(Point::new(pos.p.x, pos.p.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < w && p.y >= 0 && p.y < h);

            if player.get(ent).is_some() {
                for vis in &viewshed.visible_tiles {
                    map.revealed[[vis.x as usize, vis.y as usize]] = true;
                }
            }
        }
    }
}
