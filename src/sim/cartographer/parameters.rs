//! Input parameters.

// use crate::fmt_report;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        // fmt_report!(fmt, self.sett, "settings");
        // fmt_report!(fmt, self.tree, "tree settings");
        // fmt_report!(fmt, self.grid, "grid settings");
        Ok(())
    }
}
