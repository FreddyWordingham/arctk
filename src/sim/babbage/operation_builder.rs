//! Operation implementation.

use crate::fmt_report;
use crate::{
    data::Table,
    err::Error,
    file::{Build, Load, Redirect},
    geom::GridBuilder,
    math::Pos3,
    ord::{X, Y, Z},
    sim::babbage::Operation,
};
use arctk_attr::load;
use ndarray::Array3;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};

/// Possible operation enumeration.
#[load]
#[derive(Clone)]
pub enum OperationBuilder {
    /// Generate a zero cube of the giver resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the giver resolution.
    Unit([usize; 3]),
    /// Sum cubes together.
    Sum(Vec<PathBuf>),
    /// Add a value to the data cube.
    Add(PathBuf, f64),
    /// Subtract a value from the data cube.
    Sub(PathBuf, f64),
    /// Multiply the datacube by the value.
    Mult(PathBuf, f64),
    /// Divide the datacube by the value.
    Div(PathBuf, f64),
    /// Normalise a data cube.
    Norm(PathBuf),
    /// Sample the locations for their values. (Points, DataCube, Grid).
    Sample(PathBuf, PathBuf, Redirect<GridBuilder>),
}

impl Build for OperationBuilder {
    type Inst = Operation;

    /// Build a usable instance.
    #[inline]
    fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Zero(res) => Self::Inst::Zero(res),
            Self::Unit(res) => Self::Inst::Unit(res),
            Self::Sum(data_paths) => {
                let mut cubes = Vec::with_capacity(data_paths.len());
                for d in &data_paths {
                    cubes.push(Array3::load(&in_dir.join(d))?);
                }
                Self::Inst::Sum(cubes)
            }
            Self::Add(data_path, x) => {
                let cube = Array3::load(&in_dir.join(data_path))?;
                Self::Inst::Add(cube, x)
            }
            Self::Sub(data_path, x) => {
                let cube = Array3::load(&in_dir.join(data_path))?;
                Self::Inst::Sub(cube, x)
            }
            Self::Mult(data_path, x) => {
                let cube = Array3::load(&in_dir.join(data_path))?;
                Self::Inst::Mult(cube, x)
            }
            Self::Div(data_path, x) => {
                let cube = Array3::load(&in_dir.join(data_path))?;
                Self::Inst::Div(cube, x)
            }
            Self::Norm(data_path) => {
                let cube = Array3::load(&in_dir.join(data_path))?;
                Self::Inst::Norm(cube)
            }
            Self::Sample(points_path, data_path, grid) => {
                let table = Table::load(&in_dir.join(points_path))?;
                let points = table
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                let data_cube = Array3::load(&in_dir.join(data_path))?;

                let grid = grid.build(in_dir)?.build(in_dir)?;

                Self::Inst::Sample(points, data_cube, grid)
            }
        })
    }
}

impl Display for OperationBuilder {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::Zero(res) => {
                write!(fmt, "Zero: [{}x{}x{}]", res[X], res[Y], res[Z])
            }
            Self::Unit(res) => {
                write!(fmt, "Unit: [{}x{}x{}]", res[X], res[Y], res[Z])
            }
            Self::Sum(data_paths) => {
                write!(fmt, "Sum: [")?;
                for path in data_paths {
                    write!(fmt, "{} ", path.display())?;
                }
                write!(fmt, "]")
            }
            Self::Add(data_path, x) => {
                write!(fmt, "Add: {} + {}", data_path.display(), x)
            }
            Self::Sub(data_path, x) => {
                write!(fmt, "Add: {} - {}", data_path.display(), x)
            }
            Self::Mult(data_path, x) => {
                write!(fmt, "Add: {} * {}", data_path.display(), x)
            }
            Self::Div(data_path, x) => {
                write!(fmt, "Add: {} / {}", data_path.display(), x)
            }
            Self::Norm(data_path) => {
                write!(fmt, "Normalise: {}", data_path.display())
            }
            Self::Sample(points_path, data_path, grid) => {
                writeln!(fmt, "Sample...")?;
                fmt_report!(fmt, points_path.display(), "points");
                fmt_report!(fmt, data_path.display(), "datacube");
                fmt_report!(fmt, grid, "grid");
                Ok(())
            }
        }
    }
}
