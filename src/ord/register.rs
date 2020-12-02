//! Name register set.

use crate::ord::Name;

/// Name to index mapping.
#[derive(PartialEq)]
pub struct Register(Vec<Name>);

impl Register {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut names: Vec<Name>) -> Self {
        debug_assert!(!names.is_empty());

        names.sort();
        names.dedup();

        Self(names)
    }

    /// Reference the internal list.
    #[inline]
    #[must_use]
    pub const fn list(&self) -> &Vec<Name> {
        &self.0
    }

    /// Determine the index of a given name.
    #[allow(clippy::ptr_arg)]
    #[inline]
    #[must_use]
    pub fn index_of(&self, name: &Name) -> usize {
        self.0
            .binary_search(name)
            .unwrap_or_else(|_| panic!("Name {} not found in register", name))
    }
}
