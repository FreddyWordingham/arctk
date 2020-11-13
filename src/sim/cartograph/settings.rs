//! Runtime settings structure.

use super::{Caster, SuperSample};
use crate::{access, clone};
use arctk_attr::load;

/// Runtime settings.
#[load]
pub struct Settings {
    /// Number of samples to simulate in each thread block.
    block_size: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Optional super_sampling settings.
    super_sampling: SuperSample,
    /// Caster type.
    caster: Caster,
}

impl Settings {
    clone!(block_size, u64);
    clone!(bump_dist, f64);
    access!(super_sampling, SuperSample);
    access!(caster, Caster);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        block_size: u64,
        bump_dist: f64,
        super_sampling: SuperSample,
        caster: Caster,
    ) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);

        Self {
            block_size,
            bump_dist,
            super_sampling,
            caster,
        }
    }
}
