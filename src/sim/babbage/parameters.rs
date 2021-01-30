//! Runtime parameters.

use crate::{fmt_reports, sim::babbage::Operation};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct Parameters {
    /// Operation to perform.
    pub ops: Vec<Operation>,
}

impl Parameters {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ops: Vec<Operation>) -> Self {
        Self { ops }
    }
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_reports!(fmt, &self.ops, "operation");
        Ok(())
    }
}
