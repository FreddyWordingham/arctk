//! Loadable parameters.

use crate::{
    err::Error,
    fmt_report,
    fs::Load,
    sim::babbage::{OperationLoader, ParametersBuilder},
};
use arctk_attr::load;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable runtime parameters.
#[load]
pub struct ParametersLoader {
    /// Operation to perform.
    op: OperationLoader,
}

impl Load for ParametersLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(self.op.load(in_dir)?))
    }
}

impl Display for ParametersLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.op, "operation loader");
        Ok(())
    }
}
