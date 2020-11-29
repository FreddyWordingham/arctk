//! Walk-movement system

use crate::game::{LeftWalker, Position};
use specs::{Join, ReadStorage, System, WriteStorage};

/// left walking system.
pub struct WalkLeft {
    /// Map width.
    width: i32,
    /// Map height.
    height: i32,
}

impl WalkLeft {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(width: i32, height: i32) -> Self {
        debug_assert!(width > 0);
        debug_assert!(height > 0);

        Self { width, height }
    }
}

impl<'a> System<'a> for WalkLeft {
    type SystemData = (ReadStorage<'a, LeftWalker>, WriteStorage<'a, Position>);

    #[inline]
    fn run(&mut self, (lefty, mut pos): Self::SystemData) {
        for (_lefty, pos) in (&lefty, &mut pos).join() {
            if pos.p.x == 0 {
                pos.p.x = self.width - 1;
            } else {
                pos.p.x -= 1;
            }
        }
    }
}
