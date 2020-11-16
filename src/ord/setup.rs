//! Setup trait.

use crate::{err::Error, ord::Set};

/// Types implementing this trait can be setup to referencing a map.
pub trait Setup<'a, T> {
    type Inst;

    /// Build the instance type.
    /// # Errors
    /// if a field could not be referenced.
    fn setup(self, set: &'a Set<T>) -> Result<Self::Inst, Error>;
}
