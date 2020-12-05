//! Register structure.

use crate::ord::{Map, Name, Set};

/// Register used to index named data.
pub struct Register(Set<usize>);

impl Register {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut names: Vec<Name>) -> Self {
        debug_assert!(!names.is_empty());

        names.sort();
        names.dedup();

        let mut map = Map::new();
        for (i, name) in names.iter().enumerate() {
            map.insert(name.clone(), i);
        }

        Self(Set::new(map))
    }

    /// Find if the number of entries is zero.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Get the number of entries.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Access the internal set.
    #[inline]
    #[must_use]
    pub const fn set(&self) -> &Set<usize> {
        &self.0
    }
}
