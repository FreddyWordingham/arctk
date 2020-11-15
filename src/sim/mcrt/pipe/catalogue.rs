//! Known items.

use crate::ord::Register;

/// Catalogue of relevant registers.
pub struct Catalogue {
    /// Known surfaces.
    pub surfs: Register,
    /// Known materials.
    pub mats: Register,
    /// Known attributes.
    pub attrs: Register,
}

impl Catalogue {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(surfs: Register, mats: Register, attrs: Register) -> Self {
        Self { surfs, mats, attrs }
    }
}
