//! Operation implementation.

use crate::{
    data::Table,
    err::Error,
    fmt_report,
    fs::{File, Load, Redirect},
    geom::GridBuilder,
    math::Pos3,
    ord::{X, Y, Z},
    sim::babbage::OperationBuilder,
};
use arctk_attr::file;
use ndarray::Array3;
use std::{
    fmt::{Display, Formatter},
    path::{Path, PathBuf},
};

/// Possible operation enumeration.
#[file]
#[derive(Clone)]
pub enum OperationBuilderLoader {
    /// Report information about a data cube.
    Info(PathBuf),
    /// Generate a zero cube of the given resolution.
    Zero([usize; 3]),
    /// Generate a unit cube of the given resolution.
    Unit([usize; 3]),
    /// Generate a zero cube, with a point at the center, of the given resolution.
    Point([usize; 3]),
    /// Generate a partially filled cube, with a range of indices, within the given resolution.
    Fill {
        res: [usize; 3],
        mins: [usize; 3],
        maxs: [usize; 3],
    },
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

impl Load for OperationBuilderLoader {
    type Inst = OperationBuilder;

    #[inline]
    fn load(self, in_dir: &Path) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Info(data_path) => {
                let cube = Array3::new_from_file(&in_dir.join(data_path))?;
                Self::Inst::Info(cube)
            }
            Self::Zero(res) => Self::Inst::Zero(res),
            Self::Unit(res) => Self::Inst::Unit(res),
            Self::Point(res) => Self::Inst::Point(res),
            Self::Fill { res, mins, maxs } => Self::Inst::Fill { res, mins, maxs },
            Self::Sum(data_paths) => {
                let mut cubes = Vec::with_capacity(data_paths.len());
                for d in &data_paths {
                    cubes.push(Array3::new_from_file(&in_dir.join(d))?);
                }
                Self::Inst::Sum(cubes)
            }
            Self::Add(data_path, x) => {
                let cube = Array3::new_from_file(&in_dir.join(data_path))?;
                Self::Inst::Add(cube, x)
            }
            Self::Sub(data_path, x) => {
                let cube = Array3::new_from_file(&in_dir.join(data_path))?;
                Self::Inst::Sub(cube, x)
            }
            Self::Mult(data_path, x) => {
                let cube = Array3::new_from_file(&in_dir.join(data_path))?;
                Self::Inst::Mult(cube, x)
            }
            Self::Div(data_path, x) => {
                let cube = Array3::new_from_file(&in_dir.join(data_path))?;
                Self::Inst::Div(cube, x)
            }
            Self::Norm(data_path) => {
                let cube = Array3::new_from_file(&in_dir.join(data_path))?;
                Self::Inst::Norm(cube)
            }
            Self::Sample(points_path, data_path, grid) => {
                let table = Table::new_from_file(&in_dir.join(points_path))?;
                let points = table
                    .into_inner()
                    .iter()
                    .map(|row| Pos3::new(row[X], row[Y], row[Z]))
                    .collect();

                let data_cube = Array3::new_from_file(&in_dir.join(data_path))?;

                let grid = grid.load(in_dir)?;

                Self::Inst::Sample(points, data_cube, grid)
            }
        })
    }
}

impl Display for OperationBuilderLoader {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Info(ref data_path) => {
                write!(fmt, "Information: {}", data_path.display())
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
            Self::Fill { res, mins, maxs } => {
                write!(
                    fmt,
                    "Fill: [{} x {} x {}], [{}-{} x {}-{} x {}-{}]",
                    res[X], res[Y], res[Z], mins[X], mins[Y], mins[Z], maxs[X], maxs[Y], maxs[Z],
                )
            }
            Self::Sum(ref data_paths) => {
                write!(fmt, "Sum: [")?;
                for path in data_paths {
                    write!(fmt, "{} ", path.display())?;
                }
                write!(fmt, "]")
            }
            Self::Add(ref data_path, x) => {
                write!(fmt, "Add: {} + {}", data_path.display(), x)
            }
            Self::Sub(ref data_path, x) => {
                write!(fmt, "Subtract: {} - {}", data_path.display(), x)
            }
            Self::Mult(ref data_path, x) => {
                write!(fmt, "Multiply: {} * {}", data_path.display(), x)
            }
            Self::Div(ref data_path, x) => {
                write!(fmt, "Divide: {} / {}", data_path.display(), x)
            }
            Self::Norm(ref data_path) => {
                write!(fmt, "Normalise: {}", data_path.display())
            }
            Self::Sample(ref points_path, ref data_path, ref grid) => {
                writeln!(fmt, "Sample...")?;
                fmt_report!(fmt, points_path.display(), "points");
                fmt_report!(fmt, data_path.display(), "datacube");
                fmt_report!(fmt, grid, "grid");
                Ok(())
            }
        }
    }
}
