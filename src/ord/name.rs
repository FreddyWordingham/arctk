//! Name type.

use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error, Formatter};

/// Human-readable identifier type.
#[derive(Debug, PartialEq, Clone, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Name(String);

impl Display for Name {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        write!(fmt, "{}", self.0)
    }
}
