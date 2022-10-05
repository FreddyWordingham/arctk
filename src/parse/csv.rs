//! Comma Separated Values.

use ndarray::Array2;
use num_traits::Zero;
use std::{fs, path::Path, str::FromStr};

/// Load a two-dimensional array from a CSV file.
#[inline]
#[must_use]
pub fn load<T: Clone + Zero + FromStr>(path: &Path) -> Array2<T> {
    read(
        &fs::read_to_string(path)
            .unwrap_or_else(|_| panic!("Failed to read file: {}.", path.display())),
    )
}

/// Load a two-dimensional array from a CSV string.
/// The first line of the string must contain the column headers.
#[inline]
#[must_use]
pub fn read<T: Clone + Zero + FromStr>(s: &str) -> Array2<T> {
    let mut lines: Vec<_> = s.lines().filter(|line| !line.starts_with("//")).collect();
    let headings: Vec<_> = lines
        .remove(0)
        .split(',')
        .map(|s| (*s).to_string())
        .collect();

    let mut data = Array2::zeros((headings.len(), lines.len()));
    for (j, line) in lines.into_iter().enumerate() {
        let row: Vec<_> = line
            .split(',')
            .map(str::parse)
            .filter_map(Result::ok)
            .collect();

        if row.len() != headings.len() {
            panic!("There must be the same number of columns in each row.\nThe header contains {} columns, but line {} contains {} columns.", headings.len(), line, row.len());
        }

        for (i, elem) in row.into_iter().enumerate() {
            data[(i, j)] = elem;
        }
    }

    data
}
