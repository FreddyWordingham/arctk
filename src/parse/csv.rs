//! Commer Separated Values.

use std::{fs, path::Path};

/// Load a two-dimensional array from a CSV file.
#[inline]
#[must_use]
pub fn load<T>(path: &Path) -> Array2<T> {
    read(
        &fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}.", path.display())),
    )
}

/// Load a two-dimensional array from a CSV string.
#[inline]
#[must_use]
pub fn read<T>(s: &str) -> Array2<T> {
    println!("Reading CSV file...");
    println!("{}", s);
}
