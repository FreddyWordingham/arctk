//! Rendering simulation structure.

use crate::{clone, display_field, X, Y};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable render settings structure.
#[load]
pub struct System {
    /// Screen resolution.
    resolution: [i32; 2],
}

impl System {
    clone!(resolution, [i32; 2]);
}

impl Display for System {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field!(
            fmt,
            "resolution",
            &format!("{} x {}", self.resolution[X], self.resolution[Y])
        )
    }
}
