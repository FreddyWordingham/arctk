//! Build trait.

use crate::err::Error;
use std::path::Path;

/// Types implementing this trait can be built into another type by loading in additional resources.
pub trait Load {
    /// End type to be built.
    type Inst;

    /// Build the instance type.
    /// # Errors
    /// if a component could not be built successfully.
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error>;
}
