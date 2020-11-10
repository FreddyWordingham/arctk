//! Image alias.

use crate::{
    access,
    ord::{X, Y},
};
use ndarray::Array2;
use palette::{Gradient, LinSrgba};
use std::ops::AddAssign;

/// Image builder.
pub struct Image {
    /// Pixel data.
    pixels: Array2<LinSrgba>,
}

impl Image {
    access!(pixels, pixels_mut, Array2<LinSrgba>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2], base: LinSrgba) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            pixels: Array2::from_elem(res, base),
        }
    }

    /// Construct a new instance from numerical data and a gradient.
    #[inline]
    #[must_use]
    pub fn new_from_data(data: &Array2<f64>, grad: &Gradient<LinSrgba>) -> Self {
        Self {
            pixels: data.map(|x| grad.get(*x as f32)),
        }
    }
}

impl AddAssign<&Self> for Image {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.pixels += &rhs.pixels;
    }
}
