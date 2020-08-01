//! Image alias.

use crate::{access, Error, Save, X, Y};
use ndarray::{Array2, ShapeBuilder};
use palette::{LinSrgba, Pixel, Srgba};
use png::{BitDepth, ColorType, Encoder};
use slice_of_array::prelude::*;
use std::{fs::File, io::BufWriter, ops::AddAssign, path::Path};

/// Image wrapper.
pub struct Image {
    /// Image data.
    pixels: Array2<LinSrgba>,
}

impl Image {
    access!(pixels, pixels_mut, Array2<LinSrgba>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(res: [usize; 2]) -> Self {
        debug_assert!(res[X] > 0);
        debug_assert!(res[Y] > 0);

        Self {
            pixels: Array2::default(res),
        }
    }
}

impl AddAssign<&Self> for Image {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.pixels += &rhs.pixels;
    }
}

impl Save for Image {
    #[inline]
    #[must_use]
    fn save(&self, path: &Path) -> Result<(), Error> {
        let res = (self.pixels.shape()[0], self.pixels.shape()[1]);
        let mut data: Array2<[u8; 4]> = Array2::from_elem((res.0, res.1).f(), [0; 4]);
        for xi in 0..res.0 {
            for yi in 0..res.1 {
                let col = self.pixels[(xi, yi)];
                data[(xi, res.1 - yi - 1)] = Srgba::from_linear(col).into_format().into_raw();
            }
        }

        let file = File::create(path)?;
        let w = BufWriter::new(file);

        let mut encoder = Encoder::new(w, res.0 as u32, res.1 as u32);
        encoder.set_color(ColorType::RGBA);
        encoder.set_depth(BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        writer.write_image_data(data.into_raw_vec().flat())?;

        Ok(())
    }
}
