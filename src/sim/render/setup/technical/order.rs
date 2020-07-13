//! Rendering order enumeration implementation.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering order.
#[load]
#[derive(Clone)]
pub enum Order {
    /// Forward.
    Forward,
    /// Backward.
    Backward,
    /// Shuffle.
    Shuffle,
}

impl Display for Order {
    #[allow(clippy::result_expect_used)]
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Forward => "Forward".to_string(),
            Self::Backward => "Backward".to_string(),
            Self::Shuffle => "Shuffle".to_string(),
        };
        write!(fmt, "{}", kind)
    }
}
