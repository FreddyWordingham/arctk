//! Setup trait.

use crate::ord::Register;

/// Types implementing this trait can be built into an indexed type.
pub trait Setup {
    /// End type to be built.
    type Inst;

    /// Register the instance type to complete setup.
    fn setup(self, reg: &Register) -> Self::Inst;
}
