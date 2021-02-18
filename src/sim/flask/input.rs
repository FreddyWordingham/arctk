//! Simulation input.

use crate::{chem::Reactor, fmt_report, ord::Register, sim::flask::Settings};
use ndarray::Array1;
use std::fmt::{Display, Error, Formatter};

/// Flask simulation resources conglomerate.
pub struct Input<'a> {
    /// Register of known species.
    pub specs: &'a Register,
    /// Sources.
    pub sources: &'a Array1<f64>,
    /// Reactor processor.
    pub reactor: &'a Reactor,
    /// General settings.
    pub sett: &'a Settings,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(
        specs: &'a Register,
        sources: &'a Array1<f64>,
        reactor: &'a Reactor,
        sett: &'a Settings,
    ) -> Self {
        Self {
            specs,
            sources,
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
        fmt_report!(fmt, self.sources, "sources");
        fmt_report!(fmt, self.reactor, "reactor");
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
