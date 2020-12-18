//! Build trait.

/// Types implementing this trait can be built into another type.
pub trait Build {
    /// End type to be built.
    type Inst;

    /// Build the instance type.
    fn build(self) -> Self::Inst;
}
