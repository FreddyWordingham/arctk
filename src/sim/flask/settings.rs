//! Flask settings.

use crate::{clone, fmt_report};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
    /// Total integration time [s].
    time: f64,
    /// Number of intermediate dumps.
    dumps: usize,
}

impl Settings {
    clone!(time, f64);
    clone!(dumps, usize);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(time: f64, dumps: usize) -> Self {
        debug_assert!(time > 0.0);
        debug_assert!(dumps > 0);

        Self { time, dumps }
    }
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.time, "integration time (s)");
        fmt_report!(fmt, self.dumps, "intermediate dumps");
        Ok(())
    }
}
