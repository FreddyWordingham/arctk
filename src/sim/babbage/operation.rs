//! Operation implementation.

use crate::{
    data::Table,
    err::Error,
    file::Save,
    fmt_report, fmt_reports,
    geom::Grid,
    math::Pos3,
    ord::{X, Y, Z},
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::fmt::{Display, Formatter};
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
                Table::new(vec!["weight".to_string()], weights).save(&out_dir.join("output.csv"))
            }
        }
    }
}

impl Display for Operation {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Zero(res) => {
                write!(fmt, "Zero: [{}x{}x{}]", res[X], res[Y], res[Z])
            }
            Self::Unit(res) => {
                write!(fmt, "Unit: [{}x{}x{}]", res[X], res[Y], res[Z])
            }
            Self::Sum(cubes) => {
                writeln!(fmt, "Sum...")?;
                for cube in cubes {
                    cube_info(fmt, cube);
                }
                Ok(())
            }
            Self::Add(cube, x) => {
                writeln!(fmt, "Add...")?;
                cube_info(fmt, cube);
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Sub(cube, x) => {
                writeln!(fmt, "Subtract...")?;
                cube_info(fmt, cube);
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Mult(cube, x) => {
                writeln!(fmt, "Multiply...")?;
                cube_info(fmt, cube);
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Div(cube, x) => {
                writeln!(fmt, "Divide...")?;
                cube_info(fmt, cube);
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Norm(cube) => {
                writeln!(fmt, "Normalise...")?;
                cube_info(fmt, cube);
                Ok(())
            }
            Self::Sample(points, cube, grid) => {
                writeln!(fmt, "Normalise...")?;
                fmt_reports!(fmt, points, "sampling points");
                cube_info(fmt, cube);
                fmt_report!(fmt, grid, "grid");
                Ok(())
            }
        }
    }
}

fn cube_info(fmt: &mut Formatter, _datacube: &Array3<f64>) -> Result<(), std::fmt::Error> {
    writeln!(fmt, "...")?;
    // fmt_report!(fmt, points_path.display(), "points");
    // fmt_report!(fmt, data_path.display(), "datacube");
    // fmt_report!(fmt, grid, "grid");
    Ok(())
}
