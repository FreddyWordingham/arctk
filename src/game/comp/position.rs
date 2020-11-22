//! Position component.

use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Spatial positioning.
#[derive(Component)]
pub struct Position {
    /// Horizontal component.
    pub x: i32,
    /// Vertical component.
    pub y: i32,
}

impl Position {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
