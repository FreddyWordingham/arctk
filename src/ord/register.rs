//! Register structure.

use crate::math::is_ascending;
use std::ops::Index;

/// Register used to store named data.
pub struct Register(Vec<String>);

impl Register {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(names: Vec<String>) -> Self {
        debug_assert!(!names.is_empty());
        debug_assert!(is_ascending(names.as_slice()));

        Self(names)
    }

    /// Determine the index of a given name.
    #[inline]
    #[must_use]
    pub fn index_of(&self, name: &str) -> usize {
        self.0.iter().position(|n| n == name).unwrap()
    }
}

impl Index<usize> for Register {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.0.len());
        &self.0[index]
    }
}
