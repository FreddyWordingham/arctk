//! Position component.

use crate::math::Pos2I;
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Spatial positioning.
#[derive(Component)]
pub struct Position {
    /// Position.
    pub p: Pos2I,
}

impl Position {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            p: Pos2I::new(x, y),
        }
    }
}
