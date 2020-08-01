//! Save trait.

use crate::{Error, X, Y, Z};
use ndarray::{Array2, Array3};
use serde::Serialize;
use serde_json::to_string;
use std::{fs::write, path::Path};

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

impl<T: netcdf::Numeric> Save for Array2<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file = netcdf::create(path)?;

        let shape = self.shape();

        let dim1_name = "x";
        file.add_dimension(dim1_name, shape[X])?;
        let dim2_name = "y";
        file.add_dimension(dim2_name, shape[Y])?;

        let mut var = file.add_variable::<T>("data", &[dim1_name, dim2_name])?;
        var.put_values(self.as_slice().ok_or("Missing slice data.")?, None, None)?;

        Ok(())
    }
}

impl<T: netcdf::Numeric> Save for Array3<T> {
    #[inline]
    fn save(&self, path: &Path) -> Result<(), Error> {
        let mut file = netcdf::create(path)?;

        let shape = self.shape();

        let dim1_name = "x";
        file.add_dimension(dim1_name, shape[X])?;
        let dim2_name = "y";
        file.add_dimension(dim2_name, shape[Y])?;
        let dim3_name = "z";
        file.add_dimension(dim3_name, shape[Z])?;

        let mut var = file.add_variable::<T>("data", &[dim1_name, dim2_name, dim3_name])?;
        var.put_values(self.as_slice().ok_or("Missing slice data.")?, None, None)?;

        Ok(())
    }
}
