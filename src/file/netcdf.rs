//! Netcdf file handling.

use crate::{
    err::Error,
    ord::{X, Y, Z},
    Load, Save,
};
use ndarray::{Array2, Array3, ArrayD};
use std::path::Path;

#[allow(clippy::use_self)]
impl<T: netcdf::Numeric> Load for ArrayD<T> {
    #[inline]
    fn load(path: &Path) -> Result<ArrayD<T>, Error> {
        let file = netcdf::open(path)?;
        let var = &file.variable("data").ok_or("Missing variable 'data'.")?;
        let arr = var.values::<T>(None, None)?;
        Ok(arr)
    }
}

#[allow(clippy::use_self)]
impl<T: netcdf::Numeric> Load for Array2<T> {
    #[inline]
    fn load(path: &Path) -> Result<Array2<T>, Error> {
        let arr_d = ArrayD::load(path)?;

        let xi = arr_d.shape()[X];
        let yi = arr_d.shape()[Y];

        let arr = Array2::from_shape_vec([xi, yi], arr_d.into_raw_vec())?;
        Ok(arr)
    }
}

#[allow(clippy::use_self)]
impl<T: netcdf::Numeric> Load for Array3<T> {
    #[inline]
    fn load(path: &Path) -> Result<Array3<T>, Error> {
        let arr_d = ArrayD::load(path)?;

        let xi = arr_d.shape()[X];
        let yi = arr_d.shape()[Y];
        let zi = arr_d.shape()[Z];

        let arr = Array3::from_shape_vec([xi, yi, zi], arr_d.into_raw_vec())?;
        Ok(arr)
    }
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
