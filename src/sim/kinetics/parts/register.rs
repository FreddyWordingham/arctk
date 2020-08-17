//! Chemical name register implementation.

use crate::access;

/// Chemical name conversion handler.
pub struct Register {
    /// Chemical names.
    names: Vec<String>,
}

impl Register {
    access!(names, Vec<String>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(names: Vec<String>) -> Self {
        debug_assert!(!names.is_empty());

        Self { names }
    }
}
