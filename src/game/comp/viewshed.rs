//! Observable area.

use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Field of view.
#[derive(Component)]
pub struct Viewshed {
    /// Maximum view range.
    pub range: i32,
}

impl Viewshed {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(range: i32) -> Self {
        debug_assert!(range > 0);

        Self { range }
    }
}
