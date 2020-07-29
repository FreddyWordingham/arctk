//! Surface attributes.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Surface attributes.
#[load]
pub enum Attributes {
    /// Two-sided Mirror.
    Mirror {
        /// Absorption fraction.
        abs: f64,
    },
}

impl Display for Attributes {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Mirror { abs } => format!("Mirror: [{}]", abs),
        };
        write!(fmt, "{}", kind)
    }
}
