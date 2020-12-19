//! Loadable parameters.

use crate::{
    err::Error,
    // fmt_report,
    fs::Load,
    sim::cartographer::ParametersBuilder,
};
use arctk_attr::file;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Loadable runtime parameters.
#[file]
pub struct ParametersLoader {}

impl Load for ParametersLoader {
    type Inst = ParametersBuilder;

    #[inline]
    fn load(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(Self::Inst::new())
    }
}

impl Display for ParametersLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        // fmt_report!(fmt, self.op, "operation loader");
        Ok(())
    }
}
