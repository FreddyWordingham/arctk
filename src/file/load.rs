//! Load trait.

use crate::{Error, X, Y, Z};
use serde::Deserialize;
use std::{fs::read_to_string, path::Path};

/// Types implementing this trait can be loaded from a file.
pub trait Load
where
    Self: std::marker::Sized,
{
    /// Deserialize the type from a given file.
    /// # Errors
    /// if the target file can not be found,
    /// or the read string can not be serialised into an instance of the required type.
    fn load(path: &Path) -> Result<Self, Error>;
}

/// Deserialise the type in json format.
/// # Errors
/// if file can not be opened or read string can not be serialised into an instance of the required type.
#[inline]
pub fn from_json<T>(path: &Path) -> Result<T, Error>
where
    for<'de> T: Deserialize<'de>,
{
    println!("loading: {}", path.display());
    let s = read_to_string(path)?;
    Ok(json5::from_str(&s)?)
}

use ndarray::{Array2, Array3, ArrayD};
impl<T: netcdf::Numeric> Load for ArrayD<T> {
    #[allow(clippy::option_expect_used)]
    #[inline]
    fn load(path: &Path) -> Result<ArrayD<T>, Error> {
        let file = netcdf::open(path)?;
        let var = &file.variable("data").expect("Missing variable 'data'.");
        let arr = var.values::<T>(None, None)?;
        Ok(arr)
    }
}

impl<T: netcdf::Numeric> Load for Array2<T> {
    #[allow(clippy::option_expect_used)]
    #[inline]
    fn load(path: &Path) -> Result<Array2<T>, Error> {
        let arr_d = ArrayD::load(path)?;

        let xi = arr_d.shape()[X];
        let yi = arr_d.shape()[Y];

        let arr = Array2::from_shape_vec([xi, yi], arr_d.into_raw_vec())?;
        Ok(arr)
    }
}

impl<T: netcdf::Numeric> Load for Array3<T> {
    #[allow(clippy::option_expect_used)]
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
