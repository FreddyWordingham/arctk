//! Player controllable component.

use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Player controlled entity.
#[derive(Component)]
pub struct Player {}

impl Player {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
