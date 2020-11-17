//! Name register set.

/// Name to index mapping.
#[derive(PartialEq)]
pub struct Register(Vec<String>);

impl Register {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut names: Vec<String>) -> Self {
        debug_assert!(!names.is_empty());

        names.sort();
        names.dedup();

        Self(names)
    }

    /// Get the number of entries.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Determine the index of a given name.
    #[inline]
    #[must_use]
    pub fn index_of(&self, name: &String) -> usize {
        self.0
            .binary_search(name)
            .expect(&format!("Name {} not found in register", name))
    }
}
