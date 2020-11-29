//! Visibility system

use crate::game::{Position, Viewshed};
use specs::{Join, ReadStorage, System, WriteStorage};

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
    type SystemData = (WriteStorage<'a, Viewshed>, WriteStorage<'a, Position>);

    #[inline]
    fn run(&mut self, (mut viewshed, mut pos): Self::SystemData) {
        for (viewshed, pos) in (&viewshed, &mut pos).join() {}
    }
}
