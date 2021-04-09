//! Simulation input.

use crate::{fmt_report, sim::powder::Settings};
use std::fmt::{Display, Error, Formatter};

/// Powder simulation resources conglomerate.
pub struct Input<'a> {
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sett: &'a Settings) -> Self {
        Self { sett }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
