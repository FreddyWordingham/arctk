//! Validation trait.

/// Types implementing this trait can be queried for validity.
pub trait Valid {
    /// Check if the current state is valid.
    #[must_use]
    fn check(&self) -> bool;
}
