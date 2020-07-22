//! Engine selection enumeration.

use attr::load;
use std::fmt::{Display, Formatter, Result};

/// Rendering engine builder.
#[load]
pub enum Engine {
    /// Green engine.
    Green,
    /// Antler engine.
    Antler,
}

impl Display for Engine {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result {
        let kind = match self {
            Self::Green => "Green".to_string(),
            Self::Antler => "Antler".to_string(),
        };
        write!(fmt, "{}", kind)
    }
}
