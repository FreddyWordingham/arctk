//! Known items.

use crate::{access, ord::Register};

/// Catalogue of relevant registers.
pub struct Catalogue {
    /// Known surfaces.
    surfs: Register,
}

impl Catalogue {
    access!(surfs, Register);

    /// Construct a new instance.
    pub fn new(surfs: Register) -> Self {
        Self { surfs }
    }
}
