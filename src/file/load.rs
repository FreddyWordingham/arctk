//! Load trait.

use crate::err::Error;
use std::path::Path;

/// Types implementing this trait can be loaded from a file.
pub trait Load
where
    Self: std::marker::Sized,
{
    /// Report the opening of a fil (if it is a filepath) and load the data.
    /// # Errors
    /// if the target file can not be found,
    /// or the read string can not be serialised into an instance of the required type.
    #[inline]
    fn load(path: &Path) -> Result<Self, Error> {
        if path.is_file() {
            println!("Loading file: {}", path.display());
        }

        Self::load_data(path)
    }

    /// Deserialize the type from a given file.
    /// # Errors
    /// if the target file can not be found,
    /// or the read string can not be serialised into an instance of the required type.
    fn load_data(path: &Path) -> Result<Self, Error>;
}
