//! Input parameters.

// use crate::fmt_report;
use arctk_attr::load;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
#[load]
pub struct Parameters {}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        Ok(())
    }
}
