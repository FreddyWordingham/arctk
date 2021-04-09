//! Runtime parameters.

use crate::{fmt_report, sim::powder::Settings};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
#[file]
pub struct Parameters {
    /// General settings.
    pub sett: Settings,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(sett: Settings) -> Self {
        Self { sett }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.sett, "settings");
        Ok(())
    }
}
