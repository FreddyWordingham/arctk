//! Runtime parameters.

use crate::{chem::ReactorLinker, fmt_report, ord::ArrayLinker, sim::flask::Settings};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Simulation specific settings.
    pub sett: Settings,
    /// Initial values.
    pub init: ArrayLinker,
    /// Sources.
    pub sources: ArrayLinker,
    /// Sinks.
    pub sinks: ArrayLinker,
    /// Reactions.
    pub reactor: ReactorLinker,
}

impl Parameters {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub const fn new(
        sett: Settings,
        init: ArrayLinker,
        sources: ArrayLinker,
        sinks: ArrayLinker,
        reactor: ReactorLinker,
    ) -> Self {
        Self {
            sett,
            init,
            sources,
            sinks,
            reactor,
        }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.init, "initial values");
        fmt_report!(fmt, self.sources, "sources");
        fmt_report!(fmt, self.sinks, "sinks");
        fmt_report!(fmt, self.reactor, "reactor");
        Ok(())
    }
}
