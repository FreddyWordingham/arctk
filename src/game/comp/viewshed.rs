//! Observable area.

use rltk::Point;
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Field of view.
#[derive(Component)]
pub struct Viewshed {
    /// Maximum view range.
    pub range: i32,
    /// List of visible tiles.
    pub visible_tiles: Vec<Point>,
    /// Update requirement.
    pub dirty: bool,
}

impl Viewshed {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(range: i32) -> Self {
        debug_assert!(range > 0);

        Self {
            range,
            visible_tiles: vec![],
            dirty: true,
        }
    }
}
