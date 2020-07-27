//! Mapping function.

use crate::{cartographer::Input, Group, X, Y, Z};
use ndarray::Array3;

/// Map a volume of surfaces.
#[inline]
#[must_use]
pub fn map(input: &Input) -> Array3<Group> {
    let res = *input.grid.res();
    for _xi in 0..res[X] {
        for _yi in 0..res[Y] {
            for _zi in 0..res[Z] {}
        }
    }

    Array3::from_shape_vec(res, vec![]).unwrap()
}
