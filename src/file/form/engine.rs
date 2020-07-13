//! Engine selection enumeration.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering engine builder.
#[load]
pub enum Engine {
    /// Test engine.
    Test,
}

impl Display for Engine {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Test => "Test".to_string(),
        };
        write!(fmt, "{}", kind)
    }
}
