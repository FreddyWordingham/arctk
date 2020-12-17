//! Adaptive tree construction settings.

use crate::{clone, fmt_report};
use arctk_attr::load;
use std::fmt::{Display, Formatter};

/// Tree construction settings.
#[load]
pub struct TreeSettings {
    /// Target maximum number of triangles per cell.
    tar_tris: usize,
    /// Maximum mesh depth.
    max_depth: u32,
    /// Collision detection padding.
    padding: f64,
}

impl TreeSettings {
    clone!(tar_tris, usize);
    clone!(max_depth, u32);
    clone!(padding, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(tar_tris: usize, max_depth: u32, padding: f64) -> Self {
        debug_assert!(tar_tris > 0);
        debug_assert!(max_depth >= 1);
        debug_assert!(padding >= 0.0);

        Self {
            tar_tris,
            max_depth,
            padding,
        }
    }
}

impl Display for TreeSettings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.tar_tris, "target triangles");
        fmt_report!(fmt, self.max_depth, "maximum depth");
        fmt_report!(
            fmt,
            &format!("{}%", self.padding * 100.0),
            "padding percentage"
        );

        Ok(())
    }
}
