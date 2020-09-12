//! System settings structure.

use crate::{access, clone, display_field, display_field_ln, X, Y};
use arctk_attr::load;
use std::{
    fmt::{Display, Formatter, Result},
    path::PathBuf,
};

/// Loadable system settings structure.
#[load]
pub struct System {
    /// Window title.
    title: String,
    /// Frames-per-second limit.
    fps: i32,
    /// Screen resolution.
    resolution: [i32; 2],
    /// Font path.
    font: PathBuf,
}

impl System {
    clone!(fps, i32);
    clone!(resolution, [i32; 2]);
    access!(font, PathBuf);

    /// Reference the title string.
    #[inline]
    #[must_use]
    pub fn title(&self) -> &str {
        &self.title
    }
}

impl Display for System {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "window title", &self.title)?;
        display_field_ln!(fmt, "fps", self.fps)?;
        display_field_ln!(
            fmt,
            "resolution",
            &format!("{} x {}", self.resolution[X], self.resolution[Y])
        )?;
        display_field!(fmt, "font path", self.font.display())
    }
}
