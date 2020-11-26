//! Observable area.

use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Field of view.
#[derive(Component)]
pub struct Viewshed {}

impl Viewshed {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
