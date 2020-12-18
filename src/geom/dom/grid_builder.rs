//! Regular-Cartesian grid builder.

use crate::{
    access,
    err::Error,
    fmt_report,
    fs::Build,
    geom::{Cube, Grid},
    ord::{X, Y, Z},
};
use arctk_attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

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

    /// Determine the total number of cells.
    #[inline]
    #[must_use]
    pub const fn num_cells(&self) -> usize {
        self.res[X] * self.res[Y] * self.res[Z]
    }
}

impl Build for GridBuilder {
    type Inst = crate::geom::Grid;

    #[inline]
    fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Grid::new(self.boundary, self.res))
    }
}

impl Display for GridBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.boundary, "boundary");
        fmt_report!(
            fmt,
            &format!("[{}x{}x{}]", self.res[X], self.res[Y], self.res[Z]),
            "resolution"
        );
        Ok(())
    }
}
