//! Data table implementation.

use crate::{access, err::Error, fs::Save};
use std::{
    fmt::{Display, Formatter},
    fs::File,
    io::Write,
    ops::AddAssign,
    path::Path,
};

/// Table of row data.
pub struct Table<T> {
    /// Data headings.
    headings: Vec<String>,
    /// Count data.
    rows: Vec<Vec<T>>,
}

impl<T> Table<T> {
    access!(rows, Vec<Vec<T>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(headings: Vec<String>, rows: Vec<Vec<T>>) -> Self {
        debug_assert!(!headings.is_empty());
        debug_assert!(!rows.is_empty());

        let num_cols = headings.len();
        for row in &rows {
            debug_assert!(row.len() == num_cols);
        }

        Self { headings, rows }
    }

    /// Deconstruct the table and yield the inner rows vector.
    #[allow(clippy::missing_const_for_fn)]
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> Vec<Vec<T>> {
        self.rows
    }
}

impl<T: AddAssign + Clone> AddAssign<&Self> for Table<T> {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        debug_assert!(self.headings == rhs.headings);
        debug_assert!(self.rows.len() == rhs.rows.len());

        for (lhs, rhs) in self.rows.iter_mut().zip(&rhs.rows) {
            for (l, r) in lhs.iter_mut().zip(rhs) {
                *l += r.clone();
            }
        }
    }
}

impl<T: Display> Save for Table<T> {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
        let mut file = File::create(path)?;
        write!(file, "{}", self)?;
        Ok(())
    }
}

impl<T: Display> Display for Table<T> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.headings[0])?;
        for heading in self.headings.iter().skip(1) {
            write!(fmt, ",{}", heading)?;
        }
        writeln!(fmt)?;

        for row in &self.rows {
            let mut iter = row.iter();
            if let Some(x) = iter.next() {
                write!(fmt, "{:>32}", x)?;
            }

            for x in iter {
                write!(fmt, ", {:>32}", x)?;
            }
            writeln!(fmt)?;
        }

        Ok(())
    }
}
