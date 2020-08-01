//! Output structure.

use crate::{X, Y};
use ndarray::Array2;
use std::ops::AddAssign;

/// Rendering output structure.
pub struct Output {
    /// Average distance travelled by the tracers.
    ave_dist: Array2<f64>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            ave_dist: Array2::zeros(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.ave_dist += &rhs.ave_dist;
    }
}
