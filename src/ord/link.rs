//! Setup trait.

use crate::{err::Error, ord::Set};

/// Types implementing this trait can be linked to a set to produce a referenced type.
pub trait Link<'a, T> {
    /// Type to be built.
    type Inst;

    /// Get a list of all required resource keys.
    fn requires(self) -> Vec<String>;

    /// Link the instance type.
    /// # Errors
    /// if a field could not be referenced.
    fn link(self, set: &'a Set<T>) -> Result<Self::Inst, Error>;
}
