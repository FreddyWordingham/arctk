//! Attributes implementation.

use arctk_attr::load;
use std::fmt::{Display, Formatter, Result};

/// Surface attributes.
#[load]
pub enum Attributes {
    /// Mirror.
    Mirror,
    /// Refractive.
    Refractive {
        /// Internal material index.
        inside: usize,
        /// External material index.
        outside: usize,
    },
}

impl Display for Attributes {
    #[allow(clippy::expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match *self {
            Self::Mirror => "Mirror".to_string(),
            Self::Refractive {
                ref inside,
                ref outside,
            } => format!("Refractive: {}:|{}", inside, outside),
        };
        write!(fmt, "{}", kind)
    }
}
