//! Regular-Cartesian grid builder.

use crate::{
    access,
    geom::{Cube, Grid},
    ord::{X, Y, Z},
};
use arctk_attr::load;

/// Grid builder.
#[load]
#[derive(Clone)]
pub struct GridBuilder {
    /// Boundary.
    boundary: Cube,
    /// Resolution.
    res: [usize; 3],
}

impl GridBuilder {
    access!(boundary, Cube);
    access!(res, [usize; 3]);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        Self { boundary, res }
    }

    /// Build a Grid instance.
    #[inline]
    #[must_use]
    pub fn build(self) -> Grid {
        Grid::new(self.boundary, self.res)
    }

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn total_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }
}
