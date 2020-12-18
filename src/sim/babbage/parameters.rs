//! Input parameters.

use crate::fmt_report;
use crate::sim::babbage::OperationBuilder;
use arctk_attr::load;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
#[load]
pub struct Parameters {
    /// Operation to perform.
    pub op: OperationBuilder,
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.op, "operation builder");
        Ok(())
    }
}
