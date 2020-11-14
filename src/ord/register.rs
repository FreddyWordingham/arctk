//! Register structure.

/// Register used to convert between names and indices.
pub struct Register {
    /// Known names.
    names: Vec<String>,
}

impl Register {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(mut names: Vec<String>) -> Self {
        debug_assert!(!names.is_empty());

        names.sort();
        names.dedup();

        Self { names }
    }

    /// Determine the index of a given name.
    #[inline]
    #[must_use]
    pub fn index(&self, name: &str) -> usize {
        self.names.iter().position(|n| n == name).unwrap()
    }

    /// Determine the name corresponding to a given index.
    #[inline]
    #[must_use]
    pub fn name(&self, index: usize) -> &str {
        &self.names[index]
    }

    /// Find the total number of species in the register.
    #[inline]
    #[must_use]
    pub fn total(&self) -> usize {
        self.names.len()
    }
}
