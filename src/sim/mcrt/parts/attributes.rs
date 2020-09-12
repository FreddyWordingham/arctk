//! Attributes implementation.

use crate::Group;
use arctk_attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering attributes.
#[load]
pub enum Attributes {
    /// Spectrometer detector.
    Spectrometer,
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
    #[allow(clippy::expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Spectrometer => "Spectrometer".to_string(),
            Self::Mirror => "Mirror".to_string(),
            Self::Refractive { inside, outside } => format!("Refractive: {}:|{}", inside, outside),
        };
        write!(fmt, "{}", kind)
    }
}
