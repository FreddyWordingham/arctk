//! Data table implementation.

use crate::{
    access,
    err::Error,
    file::Save,
    tools::{Binner, Range},
};
use ndarray::Array1;
use std::{fs::File, io::Write, ops::AddAssign, path::Path};

/// Table of row data.
pub struct Table<T> {
    /// Count data.
    rows: Vec<Vec<T>>,
    /// Number of columns.
    cols: usize,
}

impl<T> Table<T> {
    access!(rows, Vec<Vec<T>>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(rows: Vec<Vec<T>>) -> Self {
        debug_assert!(!rows.is_empty());
        let cols = rows[0].len();
        for row in &rows {
            debug_assert!(row.len() == cols);
        }

        Self { rows, cols }
    }
}

// impl AddAssign<&Self> for Histogram {
//     #[inline]
//     fn add_assign(&mut self, rhs: &Self) {
//         debug_assert!(self.binner == rhs.binner);
//         debug_assert!(self.counts.len() == rhs.counts.len());

//         self.counts += &rhs.counts;
//     }
// }

// impl Save for Table {
//     #[inline]
//     fn save(&self, path: &Path) -> Result<(), Error> {
//         let mut file = File::create(path)?;

//         let mut center = self.binner.range().min();
//         let delta = self.binner.range().width() / (self.counts.len() - 1) as f64;
//         for count in &self.counts {
//             center += delta;
//             writeln!(file, "{:>32}, {:<32}", center, count)?;
//         }

//         Ok(())
//     }
// }
