//! Cartographer settings.

use crate::{
    access, clone,
    sim::cartographer::{Caster, SuperSample},
};
use arctk_attr::load;

/// General settings structure.
#[load]
pub struct Settings {
    /// Number of photons to simulate in each thread block.
    block_size: usize,
    /// Bump distance [m].
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Caster settings.
    caster: Caster,
    /// Super sampling.
    super_sampling: SuperSample,
}

impl Settings {
    clone!(block_size, usize);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    access!(caster, Caster);
    access!(super_sampling, SuperSample);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        block_size: usize,
        bump_dist: f64,
        loop_limit: u64,
        caster: Caster,
        super_sampling: SuperSample,
    ) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);

        Self {
            block_size,
            bump_dist,
            loop_limit,
            caster,
            super_sampling,
        }
    }
}
