//! Runtime parameters.

use crate::{
    fmt_report,
    ord::{X, Y},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Runtime parameters.
#[file]
pub struct Parameters {
    /// Simulation resolution.
    pub res: [usize; 2],
}

impl Display for Parameters {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            &format!("[{} x {}]", self.res[X], self.res[Y]),
            "resolution"
        );
        Ok(())
    }
}
