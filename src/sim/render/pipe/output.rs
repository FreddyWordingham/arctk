//! Output data structure.

use crate::{
    err::Error,
    file::Save,
    ord::{X, Y},
};
use ndarray::Array2;
use std::{ops::AddAssign, path::Path};

/// MCRT output data.
pub struct Output {
    /// Flight distances.
    pub dist: Array2<f64>,
    /// Render time.
    pub time: Array2<f64>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            dist: Array2::zeros(res),
            time: Array2::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.dist += &rhs.dist;
        self.time += &rhs.time;
    }
}

impl Save for Output {
    #[inline]
    fn save(&self, _out_dir: &Path) -> Result<(), Error> {
        Ok(())
    }
}
