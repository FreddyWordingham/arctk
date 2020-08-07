//! Attributes implementation.

use crate::Group;
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering attributes.
#[load]
pub enum Attributes {
    /// Mirror.
    Mirror,
    /// Refractive.
    Refractive {
        /// Internal material.
        inside: Group,
        /// External material.
        outside: Group,
    },
}

impl Display for Attributes {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Mirror => "Mirror".to_string(),
            Self::Refractive { inside, outside } => format!("Refractive: {}:|{}", inside, outside),
        };
        write!(fmt, "{}", kind)
    }
}
