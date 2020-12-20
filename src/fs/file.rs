//! Load from file trait.

use crate::err::Error;
use std::path::Path;

/// Types implementing this trait can be loaded from a file.
pub trait File
where
    Self: std::marker::Sized,
{
    /// Load an instance of this type from a given path.
    /// # Errors
    /// if the target file can not be found,
    /// or the read string can not be serialised into an instance of the required type.
    #[inline]
    fn new_from_file(path: &Path) -> Result<Self, Error> {
        println!("Loading: {}", path.display());
        Self::load(path)
    }

    /// Deserialize the type from a given file.
    /// # Errors
    /// if the target file can not be found,
    /// or the read string can not be serialised into an instance of the required type.
    fn load(path: &Path) -> Result<Self, Error>;
}
