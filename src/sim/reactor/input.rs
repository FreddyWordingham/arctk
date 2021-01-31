//! Simulation input.

use crate::{chem::Reactor, fmt_report, ord::Register, sim::reactor::Settings};
use std::fmt::{Display, Error, Formatter};

/// Reactor simulation resources conglomerate.
pub struct Input<'a> {
    /// Register of known species.
    pub specs: &'a Register,
    /// Reactor processor.
    pub reactor: &'a Reactor,
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(specs: &'a Register, reactor: &'a Reactor, sett: &'a Settings) -> Self {
        Self {
            specs,
            reactor,
            sett,
        }
    }
}

impl Display for Input<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.specs, "species");
        fmt_report!(fmt, self.reactor, "reactor");
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
