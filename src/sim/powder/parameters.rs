//! Runtime parameters.

use crate::{
    fmt_report,
    ord::{X, Y},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
#[file]
pub struct Parameters {
    /// Cell size (pixels).
    pub cell_size: usize,
    /// Simulation resolution.
    pub res: [usize; 2],
}

impl Display for Parameters {
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
