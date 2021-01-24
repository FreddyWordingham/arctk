//! Operation implementation.

use crate::{
    data::Table,
    err::Error,
    fmt_report, fmt_reports,
    fs::Save,
    geom::Grid,
    math::Pos3,
    ord::{X, Y, Z},
    util::datacube::display_datacube,
};
use ndarray::Array3;
use ndarray_stats::QuantileExt;
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// Possible operation enumeration.
pub enum Operation {
    /// Report information about data cube.
    Info(Array3<f64>),
    /// Generate a zero cube of the given resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the given resolution.
    Unit([usize; 3]),
    /// Generate a zero cube, with a point at the center, of the given resolution.
    Point([usize; 3]),
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
            Self::Info(ref data) => {
                let max = *data
                    .max()
                    .unwrap_or_else(|_| panic!("Failed to determine maximum value."));
                let min = *data
                    .min()
                    .unwrap_or_else(|_| panic!("Failed to determine minimum value."));
                let sum = data.sum();
                println!("Minimum value: {}", min);
                println!("Maximum value: {}", max);
                println!("Sum     value: {}", sum);
                println!("Average value: {}", sum / data.len() as f64);
                // (data / max).save(&out_dir.join("output.nc"))
                Ok(())
            }
            Self::Zero(res) => Array3::<f64>::zeros(res).save(&out_dir.join("output.nc")),
            Self::Unit(res) => (Array3::<f64>::zeros(res) + 1.0).save(&out_dir.join("output.nc")),
            Self::Point(res) => {
                let mut a = Array3::<f64>::zeros(res);
                a[[res[X] / 2, res[Y] / 2, res[Z] / 2]] = 1.0;
                a
            }
            .save(&out_dir.join("output.nc")),
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
                let max = *data
                    .max()
                    .unwrap_or_else(|_| panic!("Failed to determine maximum value."));
                (data / max).save(&out_dir.join("output.nc"))
            }
            Self::Sample(ref points, ref data, ref grid) => {
                let mut weights = Vec::with_capacity(points.len());
                for point in points {
                    let index = grid
                        .gen_index(point)
                        .unwrap_or_else(|| panic!("Failed to place point within grid."));
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
        match *self {
            Self::Info(ref cube) => {
                writeln!(fmt, "Information...")?;
                display_datacube(fmt, cube)?;
                Ok(())
            }
            Self::Zero(res) => {
                write!(fmt, "Zero: [{} x {} x {}]", res[X], res[Y], res[Z])
            }
            Self::Unit(res) => {
                write!(fmt, "Unit: [{} x {} x {}]", res[X], res[Y], res[Z])
            }
            Self::Point(res) => {
                write!(fmt, "Point: [{} x {} x {}]", res[X], res[Y], res[Z])
            }
            Self::Sum(ref cubes) => {
                writeln!(fmt, "Sum...")?;
                for cube in cubes {
                    display_datacube(fmt, cube)?;
                }
                Ok(())
            }
            Self::Add(ref cube, x) => {
                writeln!(fmt, "Add...")?;
                display_datacube(fmt, cube)?;
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Sub(ref cube, x) => {
                writeln!(fmt, "Subtract...")?;
                display_datacube(fmt, cube)?;
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Mult(ref cube, x) => {
                writeln!(fmt, "Multiply...")?;
                display_datacube(fmt, cube)?;
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Div(ref cube, x) => {
                writeln!(fmt, "Divide...")?;
                display_datacube(fmt, cube)?;
                fmt_report!(fmt, x, "x");
                Ok(())
            }
            Self::Norm(ref cube) => {
                writeln!(fmt, "Normalise...")?;
                display_datacube(fmt, cube)?;
                Ok(())
            }
            Self::Sample(ref points, ref cube, ref grid) => {
                writeln!(fmt, "Normalise...")?;
                fmt_reports!(fmt, points, "sampling points");
                display_datacube(fmt, cube)?;
                fmt_report!(fmt, grid, "grid");
                Ok(())
            }
        }
    }
}
