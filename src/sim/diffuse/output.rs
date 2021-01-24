//! Output data structure.

use crate::{
    access, clone,
    err::Error,
    fmt_report,
    fs::Save,
    geom::Cube,
    ord::{X, Y, Z},
};
use std::{
    fmt::{Display, Formatter},
    ops::AddAssign,
    path::Path,
};

/// Diffuse output data.
pub struct Output {
    /// Measured volume.
    boundary: Cube,
    /// Cell volume [m^3].
    cell_vol: f64,
}

impl Output {
    access!(boundary, Cube);
    clone!(cell_vol, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(boundary: Cube, res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        let cell_vol = boundary.vol() / (res[X] * res[Y] * res[Z]) as f64;

        Self { boundary, cell_vol }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, _rhs: &Self) {}
}

impl Save for Output {
    #[inline]
    fn save_data(&self, _out_dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}

impl Display for Output {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.boundary, "Boundary");
        fmt_report!(fmt, self.cell_vol, "Cell volume (m^3)");
        Ok(())
    }
}
