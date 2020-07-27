//! Mapping function.

use crate::{grid::Grid, Group, Mesh, Set, X, Y, Z};
use ndarray::Array3;

/// Map a volume of surfaces.
#[inline]
#[must_use]
pub fn map(grid: &Grid, _surfs: &Set<Mesh>) -> Array3<Group> {
    let res = *grid.res();
    for _xi in 0..res[X] {
        for _yi in 0..res[Y] {
            for _zi in 0..res[Z] {}
        }
    }

    Array3::from_shape_vec(res, vec![]).unwrap()
}
