//! Game window structure.

use crate::{access, X, Y};
use std::path::Path;
use tcod::{
    console::Root,
    console::{blit, Offscreen},
    input::Key,
    Console, FontLayout, FontType,
};

/// Game rendering window.
pub struct Window {
    /// Resolution.
    res: [i32; 2],
    /// Root window.
    root: Root,
    /// Back window.
    back: Offscreen,
}

impl Window {
    access!(root, root_mut, Root);
    access!(back, back_mut, Offscreen);

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

        Self {
            res,
            root,
            back: Offscreen::new(res[X], res[Y]),
        }
    }

    /// Await the next keypress.
    #[inline]
    pub fn wait_for_keypress(&mut self) -> Key {
        self.root.wait_for_keypress(true)
    }

    /// Clear the back canvas.
    #[inline]
    pub fn clear(&mut self) {
        self.back.clear()
    }

    /// Flush the root canvas.
    #[inline]
    pub fn flush(&mut self) {
        self.root.flush()
    }

    /// Blit the back screen to the root.
    #[inline]
    pub fn swap(&mut self) {
        blit(
            &self.back,
            (0, 0),
            (self.res[X], self.res[Y]),
            &mut self.root,
            (0, 0),
            1.0,
            1.0,
        );
    }
}
