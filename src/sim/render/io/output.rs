//! Output structure.

use crate::{X, Y};
use ndarray::Array2;
use palette::LinSrgba;
use std::ops::AddAssign;

/// Rendering output structure.
pub struct Output {
    /// Coloured output image.
    pub img: Array2<LinSrgba>,
}

impl Output {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            img: Array2::default(res),
        }
    }
}

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.img += &rhs.img;
    }
}
