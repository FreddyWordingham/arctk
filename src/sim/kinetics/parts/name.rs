//! Name trait.

use crate::{kinetics::Register, Error};

/// Types implementing this trait can be built at runtime from an input structure with names rather than indexes.
pub trait Name {
    /// End type to be constructed.
    type Inst;

    /// Build the instance type.
    /// # Errors
    /// if a component could not be named successfully.
    fn build(self, reg: &Register) -> Result<Self::Inst, Error>;
}
