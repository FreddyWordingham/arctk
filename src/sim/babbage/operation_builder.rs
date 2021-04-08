//! Operation builder.

use crate::{geom::GridBuilder, math::Pos3, ord::Build, sim::babbage::Operation};
use ndarray::Array3;

/// Possible operation enumeration.
pub enum OperationBuilder {
    /// Report information about datacube.
    Info(Array3<f64>),
    /// Sample the center of a datacube.
    Stripe(Array3<f64>),
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
    /// Add a value to the datacube.
    Add(Array3<f64>, f64),
    /// Subtract a value from the datacube.
    Sub(Array3<f64>, f64),
    /// Multiply the datacube by the value.
    Mult(Array3<f64>, f64),
    /// Divide the datacube by the value.
    Div(Array3<f64>, f64),
    /// Normalise a datacube.
    Norm(Array3<f64>),
    /// Clamp the values within datacube.
    Clamp(Array3<f64>, f64, f64),
    /// Piecewise multiply a datacube by another.
    PiecewiseMult(Array3<f64>, Array3<f64>),
    /// Piecewise divide a datacube by another.
    PiecewiseDiv(Array3<f64>, Array3<f64>),
    /// Sample the locations for their values. (Points, DataCube, Grid).
    Sample(Vec<Pos3>, Array3<f64>, GridBuilder),
}

impl Build for OperationBuilder {
    type Inst = Operation;

    #[inline]
    fn build(self) -> Self::Inst {
        match self {
            Self::Info(cube) => Self::Inst::Info(cube),
            Self::Stripe(cube) => Self::Inst::Stripe(cube),
            Self::Zero(res) => Self::Inst::Zero(res),
            Self::Unit(res) => Self::Inst::Unit(res),
            Self::Point(res) => Self::Inst::Point(res),
            Self::Fill { res, mins, maxs } => Self::Inst::Fill { res, mins, maxs },
            Self::Remove(a, b) => Self::Inst::Remove(a, b),
            Self::Sum(cubes) => Self::Inst::Sum(cubes),
            Self::Add(cube, x) => Self::Inst::Add(cube, x),
            Self::Sub(cube, x) => Self::Inst::Sub(cube, x),
            Self::Mult(cube, x) => Self::Inst::Mult(cube, x),
            Self::Div(cube, x) => Self::Inst::Div(cube, x),
            Self::Norm(cube) => Self::Inst::Norm(cube),
            Self::Clamp(cube, min, max) => Self::Inst::Clamp(cube, min, max),
            Self::PiecewiseMult(a, b) => Self::Inst::PiecewiseMult(a, b),
            Self::PiecewiseDiv(a, b) => Self::Inst::PiecewiseDiv(a, b),
            Self::Sample(points, cube, grid_builder) => {
                Self::Inst::Sample(points, cube, grid_builder.build())
            }
        }
    }
}
