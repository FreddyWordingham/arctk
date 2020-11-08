//! Index manipulation functions.

use crate::ord::{Y, Z};

/// Create the next three-dimensional index from the given linear index.
#[inline]
#[must_use]
pub const fn linear_to_three_dim(n: usize, res: &[usize; 3]) -> [usize; 3] {
    let zi = n % res[Z];
    let yi = (n / res[Z]) % res[Y];
    let xi = n / (res[Y] * res[Z]);

    [xi, yi, zi]
}
