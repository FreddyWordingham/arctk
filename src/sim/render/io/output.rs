//! Output structure.

use crate::{Error, X, Y};
use ndarray::{Array2, ShapeBuilder};
use palette::{LinSrgba, Pixel, Srgba};
use png::{BitDepth, ColorType, Encoder};
use slice_of_array::prelude::*;
use std::{fs::File, io::BufWriter, ops::AddAssign, path::Path};

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

    /// Save an array as a png image.
    #[allow(clippy::use_self)]
    #[inline]
    fn save_png(img: &Array2<LinSrgba>, path: &Path) -> Result<(), Error> {
        let res = (img.shape()[0], img.shape()[1]);
        let mut data: Array2<[u8; 4]> = Array2::from_elem((res.0, res.1).f(), [0; 4]);
        for xi in 0..res.0 {
            for yi in 0..res.1 {
                let col = img[(xi, yi)];
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

impl AddAssign<&Self> for Output {
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        self.img += &rhs.img;
    }
}
