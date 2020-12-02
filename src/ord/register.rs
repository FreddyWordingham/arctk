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

        Self { 0: Set::new(map) }
    }
}
