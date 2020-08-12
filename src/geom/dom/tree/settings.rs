//! Adaptive tree settings implementation.

use crate::{clone, display_field, display_field_ln};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Tree settings.
#[load]
pub struct Settings {
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
    /// Maximum mesh depth.
    max_depth: i32,
    /// Collision detection padding.
    padding: f64,
}

impl Settings {
    clone!(tar_tris, usize);
    clone!(max_depth, i32);
    clone!(padding, f64);
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(fmt, "target triangles", self.tar_tris)?;
        display_field_ln!(fmt, "max depth", self.max_depth)?;
        display_field!(fmt, "cell padding", self.padding * 100.0, "%")
    }
}
