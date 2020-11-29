//! Left-walking component.

use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Tendency to walk left when possible.
#[derive(Component)]
pub struct LeftWalker {}

impl LeftWalker {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}
