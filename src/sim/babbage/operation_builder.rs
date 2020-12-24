//! Operation builder.

use crate::{
    fmt_report, fmt_reports,
    geom::GridBuilder,
    math::Pos3,
    ord::{Build, X, Y, Z},
    sim::babbage::Operation,
    util::datacube::display_datacube,
};
use ndarray::Array3;
use std::fmt::{Display, Formatter};

/// Possible operation enumeration.
pub enum OperationBuilder {
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
    Sample(Vec<Pos3>, Array3<f64>, GridBuilder),
}

impl Build for OperationBuilder {
    type Inst = Operation;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Zero(res) => Self::Inst::Zero(res),
            Self::Unit(res) => Self::Inst::Unit(res),
            Self::Sum(cubes) => Self::Inst::Sum(cubes),
            Self::Add(cube, x) => Self::Inst::Add(cube, x),
            Self::Sub(cube, x) => Self::Inst::Sub(cube, x),
            Self::Mult(cube, x) => Self::Inst::Mult(cube, x),
            Self::Div(cube, x) => Self::Inst::Div(cube, x),
            Self::Norm(cube) => Self::Inst::Norm(cube),
            Self::Sample(points, cube, grid_builder) => {
                Self::Inst::Sample(points, cube, grid_builder.build())
            }
        }
    }
}

impl Display for OperationBuilder {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Zero(res) => {
                write!(fmt, "Zero: [{} x {} x {}]", res[X], res[Y], res[Z])
            }
            Self::Unit(res) => {
                write!(fmt, "Unit: [{} x {} x {}]", res[X], res[Y], res[Z])
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
            Self::Sample(ref points, ref cube, ref grid_builder) => {
                writeln!(fmt, "Normalise...")?;
                fmt_reports!(fmt, points, "sampling points");
                display_datacube(fmt, cube)?;
                fmt_report!(fmt, grid_builder, "grid builder");
                Ok(())
            }
        }
    }
}
