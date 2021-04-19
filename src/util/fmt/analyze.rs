//! Display trait.

/// Types implementing this trait can be analysed to produce a printable type.
pub trait Analyze {
    /// End type to be built.
    type Inst;

    /// Create a displayable instance.
    fn display(&self) -> Self::Inst;
}
