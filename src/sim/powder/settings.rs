//! Powder settings.

use crate::{
    fmt_report,
    ord::{X, Y},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
    /// Cell size (pixels).
    pub cell_size: usize,
    /// Simulation resolution.
    pub res: [usize; 2],
}

impl Settings {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(cell_size: usize, res: [usize; 2]) -> Self {
        debug_assert!(cell_size > 0);
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self { cell_size, res }
    }
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.cell_size, "cell size (pixels)");
        fmt_report!(
            fmt,
            &format!(
                "[{} x {}] = {}",
                self.res[X],
                self.res[Y],
                self.res[X] * self.res[Y]
            ),
            "sim resolution"
        );
        fmt_report!(
            fmt,
            &format!(
                "[{} x {}] = {}",
                self.res[X] * self.cell_size,
                self.res[Y] * self.cell_size,
                self.res[X] * self.res[Y] * self.cell_size * self.cell_size
            ),
            "window resolution"
        );
        Ok(())
    }
}
