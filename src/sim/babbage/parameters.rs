//! Input parameters.

use crate::{fmt_report, sim::babbage::Operation};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Operation to perform.
    pub op: Operation,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(op: Operation) -> Self {
        Self { op }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.op, "operation");
        Ok(())
    }
}
