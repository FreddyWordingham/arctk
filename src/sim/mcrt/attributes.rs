//! Surface attributes.

use crate::Group;
use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Surface attributes.
#[load]
pub enum Attributes {
    /// Material interface.
    Interface {
        /// Inside material group.
        inside: Group,
        /// Outside material group.
        outside: Group,
    },
    /// Two-sided Mirror.
    Mirror {
        /// Absorption fraction.
        abs: f64,
    },
    /// Absorber.
    Absorber,
}

impl Display for Attributes {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Interface { inside, outside } => format!("Interface: {}:|{}", inside, outside),
            Self::Mirror { abs } => format!("Mirror: [{}]", abs),
            Self::Absorber => "Absorber".to_string(),
        };
        write!(fmt, "{}", kind)
    }
}
