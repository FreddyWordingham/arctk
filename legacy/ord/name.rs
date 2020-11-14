//! Name trait.

use crate::{err::Error, ord::Register};

/// Types implementing this trait can be built at runtime from an input structure with names rather than indices.
pub trait Name {
    /// Type to be constructed.
    type Inst;

    /// Build the instance type by registering the names as indices.
    /// # Errors
    /// if a component could not be named successfully.
    fn reg(self, reg: &Register) -> Result<Self::Inst, Error>;
}
