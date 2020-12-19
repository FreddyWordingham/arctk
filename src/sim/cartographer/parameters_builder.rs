//! Buildable parameters.

use crate::{ord::Build, sim::cartographer::Parameters};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters builder.
pub struct ParametersBuilder {}

impl ParametersBuilder {
    /// Construct a new instance.
    #[must_use]
    #[inline]
    pub fn new() -> Self {
        Self {}
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        Self::Inst::new()
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        // fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
