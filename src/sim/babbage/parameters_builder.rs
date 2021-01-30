//! Buildable parameters.

use crate::{
    fmt_reports,
    ord::Build,
    sim::babbage::{OperationBuilder, Parameters},
};
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
pub struct ParametersBuilder {
    /// Operation builder.
    ops: Vec<OperationBuilder>,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ops: Vec<OperationBuilder>) -> Self {
        Self { ops }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        Self::Inst::new(self.ops.into_iter().map(Build::build).collect())
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_reports!(fmt, &self.ops, "operation builders");
        Ok(())
    }
}
