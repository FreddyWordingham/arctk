//! Portable Network Graphics.

use ndarray::{Array2, ArrayView2};
use palette::{LinSrgba, Pixel};
use png::{BitDepth, ColorType, Encoder};
use slice_of_array::SliceFlatExt;
use std::{fs::File, io::BufWriter, path::Path};

/// Save an array as a PNG file.
#[inline]
pub fn save(image: ArrayView2<LinSrgba>, path: &Path) {
    // Convert to png rgb space.
    let res = image.shape();
    let mut data: Array2<[u8; 4]> = Array2::from_elem((res[1], res[0]), [0; 4]);
    for xi in 0..res[0] {
        for yi in 0..res[1] {
            data[(res[1] - yi - 1, xi)] = image[(xi, yi)].into_format().into_raw();
        }
    }

    // Save data at path.
    let file = File::create(path)
        .unwrap_or_else(|_| panic!("Failed to create PNG file: {}", path.display()));
    let w = BufWriter::new(file);
    let mut encoder = Encoder::new(w, res[0] as u32, res[1] as u32);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(BitDepth::Eight);
    encoder
        .write_header()
        .expect("Failed to write PNG header.")
        .write_image_data(data.into_raw_vec().flat())
        .expect("Failed to write PNG data.");
}
