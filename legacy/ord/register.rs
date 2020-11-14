//! Register structure.

use crate::ord::{Key, KeyRef};
use ndarray::Array1;

/// Register used to convert between keys and indices.
pub struct Register {
    /// Known keys.
    keys: Array1<Key>,
}

impl Register {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut keys: Vec<Key>) -> Self {
        debug_assert!(!keys.is_empty());

        keys.sort();
        keys.dedup();

        Self {
            keys: Array1::from(keys),
        }
    }

    /// Determine the index of a given key.
    #[inline]
    #[must_use]
    pub fn index(&self, key: KeyRef) -> usize {
        self.keys.iter().position(|n| n == key).unwrap()
    }

    /// Determine the key corresponding to a given index.
    #[inline]
    #[must_use]
    pub fn name(&self, index: usize) -> &Key {
        &self.keys[index]
    }

    /// Find the total number of species in the register.
    #[inline]
    #[must_use]
    pub fn total(&self) -> usize {
        self.keys.len()
    }
}
