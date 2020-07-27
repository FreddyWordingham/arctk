//! Output structure.

use crate::{ord::Group, Error, Save, X, Y, Z};
use ndarray::Array3;
use std::{ops::AddAssign, path::Path};

/// Test engine output structure.
pub struct Output {
    /// Center material sample.
    pub mats: Array3<Option<Group>>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 3]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);
        debug_assert!(res[Z] > 0);

        Self {
            mats: Array3::default(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, _rhs: &Self) {}
}

impl Save for Output {
    #[inline]
    fn save(&self, out_dir: &Path) -> Result<(), Error> {
        // Get current time string.
        let time = chrono::offset::Local::now()
            .format("%Y%m%d%H%M%S")
            .to_string();
        let path = out_dir.join(time);
        std::fs::create_dir(&path)?;

        Ok(())
    }
}
