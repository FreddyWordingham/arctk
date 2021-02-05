//! Image alias.

use crate::{
    access,
    img::Colour,
    ord::{X, Y},
};
use ndarray::Array2;
use std::ops::AddAssign;

/// Image builder.
#[derive(Clone)]
pub struct Image {
    /// Pixel data.
    pixels: Array2<Colour>,
}

impl Image {
    access!(pixels, pixels_mut, Array2<Colour>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(pixels: Array2<Colour>) -> Self {
        Self { pixels }
    }

    /// Construct a new blank instance.
    #[inline]
    #[must_use]
    pub fn new_blank(res: [usize; 2], base: Colour) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            pixels: Array2::from_elem(res, base),
        }
    }
}

impl AddAssign<&Self> for Image {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.pixels += &rhs.pixels;
    }
}
