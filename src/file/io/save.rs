//! Save trait.

use crate::Error;
use ndarray::{Array2, ShapeBuilder};
use palette::{LinSrgba, Pixel, Srgba};
use png::{BitDepth, ColorType, Encoder};
use serde::Serialize;
use serde_json::to_string;
use slice_of_array::prelude::*;
use std::{
    fs::{write, File},
    io::BufWriter,
    path::Path,
};

/// Types implementing this trait can be saved to file.
pub trait Save {
    /// Serialise the type to a given file
    /// # Errors
    /// if the instance can not be serialised or if the file can't be written to.
    fn save(&self, path: &Path) -> Result<(), Error>;
}

/// Serialise the type in json format.
/// # Errors
/// if the instance can not be serialised into json or if the file can't be written to.
#[inline]
pub fn as_json<T: Serialize>(instance: &T, path: &Path) -> Result<(), Error> {
    println!("saving: {}", path.display());
    let s = to_string(instance)?;
    Ok(write(path, s)?)
}

impl Save for Array2<LinSrgba> {
    #[allow(clippy::use_self)]
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        let res = (self.shape()[0], self.shape()[1]);
        let mut data = Array2::from_elem((res.0, res.1).f(), [0; 4]);
        for xi in 0..res.0 {
            for yi in 0..res.1 {
                let col = self[(xi, yi)];
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

// impl Save for Array2<f64> {
//     #[inline]
//     fn save(&self, path: &Path) -> Result<(), Error> {
//         let mut file = netcdf::create(path)?;

//         let shape = self.shape();

//         let dim1_name = "x";
//         file.add_dimension(dim1_name, shape[X])?;
//         let dim2_name = "y";
//         file.add_dimension(dim2_name, shape[Y])?;

//         let mut var = file.add_variable::<f64>("data", &[dim1_name, dim2_name])?;
//         var.put_values(self.as_slice().ok_or("Missing slice data.")?, None, None)?;

//         Ok(())
//     }
// }

// impl Save for Array3<f64> {
//     #[inline]
//     fn save(&self, path: &Path) -> Result<(), Error> {
//         let mut file = netcdf::create(path)?;

//         let shape = self.shape();

//         let dim1_name = "x";
//         file.add_dimension(dim1_name, shape[X])?;
//         let dim2_name = "y";
//         file.add_dimension(dim2_name, shape[Y])?;
//         let dim3_name = "z";
//         file.add_dimension(dim3_name, shape[Z])?;

//         let mut var = file.add_variable::<f64>("data", &[dim1_name, dim2_name, dim3_name])?;
//         var.put_values(self.as_slice().ok_or("Missing slice data.")?, None, None)?;

//         Ok(())
//     }
// }
