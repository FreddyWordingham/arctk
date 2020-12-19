//! Input parameters.

use crate::{
    fmt_report,
    ord::Build,
    sim::babbage::{OperationBuilder, Parameters},
};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct ParametersBuilder {
    /// Operation builder.
    op: OperationBuilder,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(op: OperationBuilder) -> Self {
        Self { op }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        Self::Inst::new(self.op.build())
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.op, "operation loader");
        Ok(())
    }
}
