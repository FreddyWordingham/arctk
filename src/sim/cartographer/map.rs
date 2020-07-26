//! Mapping function.

use crate::{grid::Grid, Group, Mesh, Set};
use ndarray::Array3;

/// Map a volume of surfaces.
#[inline]
#[must_use]
pub fn map(grid: &Grid, _surfs: &Set<Mesh>) -> Array3<Group> {
    Array3::from_shape_vec(*grid.res(), vec![]).unwrap()
}
