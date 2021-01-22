//! Loadable parameters.

use crate::{
    chem::ReactorLinker,
    fmt_report,
    ord::{ArrayLinker, Build},
    sim::flask::{Parameters, Settings},
};
use std::fmt::{Display, Error, Formatter};

/// Loadable runtime parameters.
pub struct ParametersBuilder {
    /// Simulation specific settings.
    sett: Settings,
    /// Initial concentrations.
    concs: ArrayLinker,
    /// Reactions.
    reactor: ReactorLinker,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    #[inline]
    pub const fn new(sett: Settings, concs: ArrayLinker, reactor: ReactorLinker) -> Self {
        Self {
            sett,
            concs,
            reactor,
        }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let sett = self.sett;
        let concs = self.concs;
        let reactor = self.reactor;

        Self::Inst::new(sett, concs, reactor)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        fmt_report!(fmt, self.concs, "concentrations");
        fmt_report!(fmt, self.reactor, "reactor");
        Ok(())
    }
}
