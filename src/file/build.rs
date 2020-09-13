//! Build trait.

use crate::err::Error;
use std::path::Path;

/// Types implementing this trait can be built at runtime from an input form structure.
pub trait Build {
    /// End type to be built.
    type Inst;

    /// Build the instance type.
    /// # Errors
    /// if a component could not be built successfully.
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error>;
}
