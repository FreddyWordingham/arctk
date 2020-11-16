//! Rendering settings.

use crate::{access, clone, img::Gradient};

/// General settings structure.
pub struct Settings<'a> {
    /// Number of tracers to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Minimum statistical weight to consider.
    min_weight: f64,
    /// Sky colour gradient.
    sky_grad: &'a Gradient,
}

impl<'a> Settings<'a> {
    clone!(block_size, u64);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    clone!(min_weight, f64);
    access!(sky_grad, Gradient);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        block_size: u64,
        bump_dist: f64,
        loop_limit: u64,
        min_weight: f64,
        sky_grad: &'a Gradient,
    ) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);
        debug_assert!(min_weight >= 0.0);

        Self {
            block_size,
            bump_dist,
            loop_limit,
            min_weight,
            sky_grad,
        }
    }
}
