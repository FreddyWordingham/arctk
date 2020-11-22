//! Position component.

use rltk::{to_cp437, FontCharType, RGB};
use specs::{Component, DenseVecStorage};
use specs_derive::Component;

/// Rendering data.
#[derive(Component)]
pub struct Renderable {
    /// Character.
    pub glyph: FontCharType,
    /// Character colour.
    pub fg: RGB,
    /// Background colour.
    pub bg: RGB,
}

impl Renderable {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(glyph: char, fg: RGB, bg: RGB) -> Self {
        Self {
            glyph: to_cp437(glyph),
            fg,
            bg,
        }
    }
}
