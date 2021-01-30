//! Loadable parameters.

use crate::{
    err::Error,
    fmt_reports,
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
    /// Operations to perform.
    ops: Vec<OperationBuilderLoader>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new(
            self.ops
                .into_iter()
                .map(|op| op.load(in_dir))
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl Display for ParametersBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_reports!(fmt, &self.ops, "operation loaders");
        Ok(())
    }
}
