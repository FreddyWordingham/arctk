//! NetCDF file handling.

use crate::{
    err::Error,
    fs::{File, Save},
    ord::{X, Y, Z},
};
use ndarray::{Array2, Array3, ArrayView2, ArrayView3};
use netcdf::Numeric;
use std::path::Path;

#[allow(clippy::use_self)]
impl<T: Numeric> File for Array2<T> {
    #[inline]
    fn load(path: &Path) -> Result<Array2<T>, Error> {
        let file = netcdf::open(path)?;
        let data = &file.variable("data").ok_or("Missing variable 'data'.")?;
        let arr = data.values::<T>(None, None)?;

        let xi = arr.shape()[X];
        let yi = arr.shape()[Y];

        let arr = Array2::from_shape_vec([xi, yi], arr.into_raw_vec())?;
        Ok(arr)
    }
}

#[allow(clippy::use_self)]
impl<T: Numeric> File for Array3<T> {
    #[inline]
    fn load(path: &Path) -> Result<Array3<T>, Error> {
        let file = netcdf::open(path)?;
        let data = &file.variable("data").ok_or("Missing variable 'data'.")?;
        let arr = data.values::<T>(None, None)?;

        let xi = arr.shape()[X];
        let yi = arr.shape()[Y];
        let zi = arr.shape()[Z];

        let arr = Array3::from_shape_vec([xi, yi, zi], arr.into_raw_vec())?;
        Ok(arr)
    }
}

impl<T: Numeric> Save for Array2<T> {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
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

impl<T: Numeric> Save for ArrayView2<'_, T> {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
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

impl<T: Numeric> Save for Array3<T> {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
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

impl<T: Numeric> Save for ArrayView3<'_, T> {
    #[inline]
    fn save_data(&self, path: &Path) -> Result<(), Error> {
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
