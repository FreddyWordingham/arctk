//! Operation implementation.

use crate::{data::Table, err::Error, file::Save, geom::Grid, math::Pos3};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::path::Path;

/// Possible operation enumeration.
pub enum Operation {
    /// Generate a zero cube of the giver resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
    /// Sum cubes together.
    Sum(Vec<Array3<f64>>),
    /// Add a value to the data cube.
    Add(Array3<f64>, f64),
    /// Subtract a value from the data cube.
    Sub(Array3<f64>, f64),
    /// Multiply the datacube by the value.
    Mult(Array3<f64>, f64),
    /// Divide the datacube by the value.
    Div(Array3<f64>, f64),
    /// Normalise a data cube.
    Norm(Array3<f64>),
    /// Sample the locations for their values. (Points, DataCube, Grid).
    Sample(Vec<Pos3>, Array3<f64>, Grid),
}

impl Operation {
    /// Perform the operation.
    /// # Errors
    /// if an output file can not be saved successfully
    /// or if a sampling point can not be located within the grid.
    #[allow(clippy::expect_used)]
    #[inline]
    pub fn run(&self, out_dir: &Path) -> Result<(), Error> {
        match *self {
            Self::Zero(res) => Array3::<f64>::zeros(res).save(&out_dir.join("output.nc")),
            Self::Unit(res) => (Array3::<f64>::zeros(res) + 1.0).save(&out_dir.join("output.nc")),
            Self::Sum(ref data) => {
                let mut base = data[0].clone();
                for d in data.iter().skip(1) {
                    base += d;
                }
                base.save(&out_dir.join("output.nc"))
            }
            Self::Add(ref data, x) => (data + x).save(&out_dir.join("output.nc")),
            Self::Sub(ref data, x) => (data - x).save(&out_dir.join("output.nc")),
            Self::Mult(ref data, x) => (data * x).save(&out_dir.join("output.nc")),
            Self::Div(ref data, x) => (data / x).save(&out_dir.join("output.nc")),
            Self::Norm(ref data) => {
                let max = *data.max().expect("Failed to determine maximal value.");
                (data / max).save(&out_dir.join("output.nc"))
            }
            Self::Sample(ref points, ref data, ref grid) => {
                let mut weights = Vec::with_capacity(points.len());
                for point in points {
                    let index = grid
                        .gen_index(point)
                        .expect("Failed to place point within grid.");
                    weights.push(vec![data[index]]);
                }
                Table::new(weights).save(&out_dir.join("output.csv"))
            }
        }
    }
}
