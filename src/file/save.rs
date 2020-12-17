//! Save trait.

use crate::err::Error;
use serde::Serialize;
use serde_json::to_string;
use std::{fs::write, path::Path};

/// Types implementing this trait can be saved to file.
pub trait Save {
    /// Serialise the type to a given file
    /// # Errors
    /// if the instance can not be serialised or if the file can't be written to.
    fn save_data(&self, path: &Path) -> Result<(), Error>;

    /// Report the saving of a file (if it is a filepath) and save the data.
    /// # Errors
    /// if the instance can not be serialised or if the file can't be written to.
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        println!("Saving: {}", path.display());

        self.save_data(path)
    }
}

/// Serialise the type in json format.
/// # Errors
/// if the instance can not be serialised into json or if the file can't be written to.
#[inline]
pub fn as_json<T: Serialize>(instance: &T, path: &Path) -> Result<(), Error> {
    println!("saving: {}", path.display());
    let s = to_string(instance)?;
    Ok(write(path, s)?)
}
