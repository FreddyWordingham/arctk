//! Chemical name register implementation.

use crate::{access, kinetics::Chem};

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
    pub fn new(mut names: Vec<String>) -> Self {
        names.sort();
        names.dedup();

        debug_assert!(!names.is_empty());

        Self { names }
    }

    /// Retrieve the name of the given chemical.
    #[inline]
    #[must_use]
    pub fn name(&self, c: Chem) -> &str {
        &self.names[c]
    }

    /// Retrieve the chemical index for a given name.
    #[allow(clippy::expect_used)]
    #[inline]
    #[must_use]
    pub fn index(&self, name: &str) -> Chem {
        self.names
            .iter()
            .position(|n| n == name)
            .expect("Unknown chemical name.")
    }
}
