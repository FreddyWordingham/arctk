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
    path::{Path, PathBuf},
};

/// Loadable runtime parameters.
#[file]
pub struct ParametersBuilderLoader {
    /// Operations to perform, and their output path.
    ops: Vec<(OperationBuilderLoader, PathBuf)>,
}

impl Load for ParametersBuilderLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        let mut ops = Vec::with_capacity(self.ops.len());
        for (op, path) in self.ops {
            ops.push((op.load(in_dir)?, path));
        }

        Ok(Self::Inst::new(ops))
    }
}

impl Display for ParametersBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        for &(ref op, ref path) in &self.ops {
            fmt_report!(fmt, op, path.display());
        }
        Ok(())
    }
}
