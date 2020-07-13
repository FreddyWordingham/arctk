//! Executable information.

use crate::Error;
use std::{env::args, path::Path};

/// Determine the name of the executable.
/// # Errors
/// if the binary name can not be identified.
#[inline]
pub fn name() -> Result<String, Error> {
    let args: Vec<String> = args().collect();

    Ok(Path::new(&args[0])
        .file_stem()
        .ok_or("Missing filename.")?
        .to_str()
        .ok_or("Missing string.")?
        .to_string())
}
