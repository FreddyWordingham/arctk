//! Index manipulation functions.

use crate::ord::{X, Y, Z};

/// Determine the linear index form a two-dimension index and resolution.
#[inline]
#[must_use]
pub fn two_dim_to_linear(pos: [usize; 2], res: &[usize; 2]) -> usize {
    debug_assert!(pos[X] < res[X]);
    debug_assert!(pos[Y] < res[Y]);

    (pos[Y] * res[Y]) + pos[X]
}

/// Create the next three-dimensional index from the given linear index.
#[inline]
#[must_use]
pub fn linear_to_three_dim(n: usize, res: &[usize; 3]) -> [usize; 3] {
    debug_assert!(n < (res[X] * res[Y] * res[Z]));

    let zi = n % res[Z];
    let yi = (n / res[Z]) % res[Y];
    let xi = n / (res[Y] * res[Z]);

    [xi, yi, zi]
}
