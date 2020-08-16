//! System settings structure.

use crate::{clone, display_field, display_field_ln, X, Y};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable system settings structure.
#[load]
pub struct System {
    /// Frames-per-second limit.
    fps: i32,
    /// Screen resolution.
    resolution: [i32; 2],
}

impl System {
    clone!(fps, i32);
    clone!(resolution, [i32; 2]);
}

impl Display for System {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "fps", self.fps)?;
        display_field!(
            fmt,
            "resolution",
            &format!("{} x {}", self.resolution[X], self.resolution[Y])
        )
    }
}
