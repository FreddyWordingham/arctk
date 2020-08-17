//! Drawing information structure.

use crate::game::torus::Coor2;
use tcod::{
    colors::WHITE,
    console::{Console, Offscreen},
    BackgroundFlag, Color,
};

/// Drawable trait.
pub trait Draw {
    /// Get the coordindates of the drawable.
    fn pos(&self) -> Coor2;

    /// Get the drawable character.
    #[inline]
    #[must_use]
    fn sym(&self) -> char {
        '*'
    }

    /// Get the drawable colour.
    #[inline]
    #[must_use]
    fn col(&self) -> Color {
        WHITE
    }

    /// Draw
    #[inline]
    fn draw(&self, window: &mut Offscreen) {
        window.set_default_foreground(self.col());

        let pos = self.pos();
        let px = pos.x;
        let py = window.height() - pos.y;
        window.put_char(px, py, self.sym(), BackgroundFlag::None);
    }
}
