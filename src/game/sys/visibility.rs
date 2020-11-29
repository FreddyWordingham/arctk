//! Visibility system

use crate::game::{Map, Position, Viewshed};
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
        for (viewshed, pos) in (&viewshed, &mut pos).join() {}
    }
}
