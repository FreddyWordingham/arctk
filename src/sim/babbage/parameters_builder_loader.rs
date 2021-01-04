//! Loadable parameters.

use crate::{
    err::Error,
    fmt_report,
    fs::Load,
    sim::babbage::{OperationBuilderLoader, ParametersBuilder},
};
use arctk_attr::file;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Operation to perform.
    op: OperationBuilderLoader,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(self.op.load(in_dir)?))
    }
}

impl Display for ParametersBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.op, "operation loader");
        Ok(())
    }
}
