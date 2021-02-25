//! Operation implementation.

use crate::{
    data::Table,
    err::Error,
    fs::Save,
    geom::Grid,
    math::Pos3,
    ord::{X, Y, Z},
    report,
    util::fmt::Analyze,
};
use ndarray::Array3;
use std::path::Path;

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
    /// Generate a partially filled cube, with a range of indices, within the given resolution.
    Fill {
        /// Total resolution.
        res: [usize; 3],
        /// Minimum inclusive filling bound.
        mins: [usize; 3],
        /// Maximum inclusive filling bound.
        maxs: [usize; 3],
    },
    /// Remove one cube from another.
    Remove(Array3<f64>, Array3<f64>),
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
    pub fn run(&self, out_dir: &Path, name: String) -> Result<(), Error> {
        let path = out_dir.join(name);

        match *self {
            Self::Info(ref data) => {
                report!(data.display(), "data");
                Ok(())
            }
            Self::Zero(res) => Array3::<f64>::zeros(res).save(&path.with_extension("nc")),
            Self::Unit(res) => (Array3::<f64>::zeros(res) + 1.0).save(&path.with_extension("nc")),
            Self::Point(res) => {
                let mut a = Array3::<f64>::zeros(res);
                a[[res[X] / 2, res[Y] / 2, res[Z] / 2]] = 1.0;
                a
            }
            .save(&path.with_extension("nc")),
            Self::Fill { res, mins, maxs } => {
                let mut a = Array3::<f64>::zeros(res);
                for zi in mins[Z]..=maxs[Z] {
                    for yi in mins[Y]..=maxs[Y] {
                        for xi in mins[X]..=maxs[X] {
                            a[[xi, yi, zi]] = 1.0;
                        }
                    }
                }
                a
            }
            .save(&path.with_extension("nc")),
            Self::Remove(ref data_a, ref data_b) => {
                (data_a - data_b).save(&path.with_extension("nc"))
            }
            Self::Sum(ref data) => {
                let mut base = data[0].clone();
                for d in data.iter().skip(1) {
                    base += d;
                }
                base.save(&path.with_extension("nc"))
            }
            Self::Add(ref data, x) => (data + x).save(&path.with_extension("nc")),
            Self::Sub(ref data, x) => (data - x).save(&path.with_extension("nc")),
            Self::Mult(ref data, x) => (data * x).save(&path.with_extension("nc")),
            Self::Div(ref data, x) => (data / x).save(&path.with_extension("nc")),
            Self::Norm(ref data) => (data / data.sum()).save(&path.with_extension("nc")),
            Self::Sample(ref points, ref data, ref grid) => {
                let mut weights = Vec::with_capacity(points.len());
                for point in points {
                    let index = grid
                        .gen_index(point)
                        .unwrap_or_else(|| panic!("Failed to place point within grid."));
                    weights.push(vec![data[index]]);
                }
                Table::new(vec!["weight".to_string()], weights).save(&path.with_extension("csv"))
            }
        }
    }
}
