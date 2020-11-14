//! Operation implementation.

use super::Operation;
use crate::{
    data::Table,
    err::Error,
    file::{Build, Load, Redirect},
    geom::GridBuilder,
    math::Pos3,
    ord::{X, Y, Z},
};
use arctk_attr::load;
use ndarray::Array3;
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

                let grid = grid.build(in_dir)?.build();

                Self::Inst::Sample(points, data_cube, grid)
            }
        })
    }
}
