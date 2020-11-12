//! Data table implementation.

use crate::{access, clone, err::Error, file::Save};
use std::{fmt::Display, fs::File, io::Write, ops::AddAssign, path::Path};

/// Table of row data.
pub struct Table<T> {
    /// Count data.
    rows: Vec<Vec<T>>,
    /// Number of columns.
    num_cols: usize,
}

impl<T> Table<T> {
    access!(rows, Vec<Vec<T>>);
    clone!(num_cols, usize);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        debug_assert!(!rows.is_empty());
        let num_cols = rows[0].len();
        for row in &rows {
            debug_assert!(row.len() == num_cols);
        }

        Self { rows, num_cols }
    }
}

impl<T: AddAssign + Clone> AddAssign<&Self> for Table<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!(self.rows.len() == rhs.rows.len());
        debug_assert!(self.num_cols == rhs.num_cols);

        for (lhs, rhs) in self.rows.iter_mut().zip(&rhs.rows) {
            for (l, r) in lhs.iter_mut().zip(rhs) {
                *l += r.clone();
            }
        }
    }
}

impl<T: Display> Save for Table<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file = File::create(path)?;

        for row in &self.rows {
            let mut iter = row.iter();
            if let Some(x) = iter.next() {
                write!(file, "{:>32}", x)?;
            }

            for x in iter {
                write!(file, ", {:>32}", x)?;
            }
            writeln!(file)?;
        }

        Ok(())
    }
}
