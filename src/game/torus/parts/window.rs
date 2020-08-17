//! Game window structure.

use crate::{access, X, Y};
use std::path::Path;
use tcod::{console::Root, Console, FontLayout, FontType};

/// Game rendering window.
pub struct Window {
    /// Root window.
    root: Root,
}

impl Window {
    access!(root, root_mut, Root);

    /// Construct a new entity.
    #[inline]
    #[must_use]
    pub fn new(title: &str, font: &Path, res: [i32; 2], fps: i32) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(fps > 0);

        let root = Root::initializer()
            .font(font, FontLayout::Tcod)
            .font_type(FontType::Greyscale)
            .size(res[X], res[Y])
            .title(title)
            .fullscreen(true)
            .init();
        tcod::system::set_fps(fps);

        Self { root }
    }

    /// Clear the root canvas.
    #[inline]
    pub fn clear(&mut self) {
        self.root.clear()
    }

    /// Flush the root canvas.
    #[inline]
    pub fn flush(&mut self) {
        self.root.flush()
    }
}
