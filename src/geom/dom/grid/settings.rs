//! Regular grid settings implementation.

use crate::{access, display_field, display_field_ln, Aabb, X, Y, Z};
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Loadable triangle mesh conglomerate structure.
#[load]
pub struct Settings {
    /// Boundary.
    boundary: Aabb,
    /// Resolution.
    res: [usize; 3],
}

impl Settings {
    access!(boundary, Aabb);
    access!(res, [usize; 3]);

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        display_field_ln!(
            fmt,
            "resolution",
            format!("{}x{}x{}", self.res[X], self.res[Y], self.res[Z])
        )?;
        display_field_ln!(fmt, "total cells", self.total_cells())?;
        display_field!(fmt, "boundary", &self.boundary)
    }
}
