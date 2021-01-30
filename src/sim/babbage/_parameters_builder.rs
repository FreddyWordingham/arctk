//! Buildable parameters.

use crate::{
    fmt_report,
    ord::Build,
    sim::babbage::{OperationBuilder, Parameters},
};
use std::{
    fmt::{Display, Error, Formatter},
    path::PathBuf,
};

/// Runtime parameters.
pub struct ParametersBuilder {
    /// Operation builder.
    ops: Vec<(OperationBuilder, PathBuf)>,
}

impl ParametersBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(ops: Vec<(OperationBuilder, PathBuf)>) -> Self {
        Self { ops }
    }
}

impl Build for ParametersBuilder {
    type Inst = Parameters;

    #[inline]
    fn build(self) -> Self::Inst {
        let mut ops = Vec::with_capacity(self.ops.len());
        for (op, path) in self.ops {
            ops.push((op.build(), path));
        }

        Self::Inst::new(ops)
    }
}

impl Display for ParametersBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        for &(ref op, ref path) in &self.ops {
            fmt_report!(fmt, op, path.display());
        }
        Ok(())
    }
}
