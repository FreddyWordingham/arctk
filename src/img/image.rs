//! Image alias.

use crate::{
    access,
    err::Error,
    ord::{X, Y},
};
use ndarray::Array2;
use ndarray_stats::QuantileExt;
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
    /// # Errors
    /// if the minimum or maximum data value can not be determined.
    #[inline]
    pub fn data_to_linear(data: &Array2<f64>, grad: &Gradient<LinSrgba>) -> Result<Self, Error> {
        let min = *data.min()?;
        let max = *data.max()?;
        let linear = (data + min) / (max - min);

        Ok(Self {
            pixels: linear.map(|x| grad.get(*x as f32)),
        })
    }

    /// Construct a new instance from numerical data and a gradient.
    /// # Errors
    /// if the minimum or maximum data value can not be determined.
    #[inline]
    pub fn data_to_log(data: &Array2<f64>, grad: &Gradient<LinSrgba>) -> Result<Self, Error> {
        let min = *data.min()?;
        let max = *data.max()?;
        let linear = (data + min) / (max - min);
        let log = linear.map(|x| x.log(10.0));

        Ok(Self {
            pixels: log.map(|x| grad.get(*x as f32)),
        })
    }
}

impl AddAssign<&Self> for Image {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.pixels += &rhs.pixels;
    }
}
