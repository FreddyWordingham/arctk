//! MCRT settings.

use crate::{access, clone, sim::mcrt::Material};

/// General settings structure.
pub struct Settings<'a> {
    /// Number of photons to simulate.
    num_phot: usize,
    /// Number of photons to simulate in each thread block.
    block_size: usize,
    /// Bump distance [m].
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Minimum statistical weight to consider.
    min_weight: f64,
    /// Number of roulette barrels.
    roulette_barrels: u64,
    /// Emission material.
    init_mat: &'a Material,
}

impl<'a> Settings<'a> {
    clone!(num_phot, usize);
    clone!(block_size, usize);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    clone!(min_weight, f64);
    clone!(roulette_barrels, u64);
    access!(init_mat, Material);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        num_phot: usize,
        block_size: usize,
        bump_dist: f64,
        loop_limit: u64,
        min_weight: f64,
        roulette_barrels: u64,
        init_mat: &'a Material,
    ) -> Self {
        debug_assert!(num_phot > 0);
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);
        debug_assert!(min_weight >= 0.0);
        debug_assert!(roulette_barrels > 1);

        Self {
            num_phot,
            block_size,
            bump_dist,
            loop_limit,
            min_weight,
            roulette_barrels,
            init_mat,
        }
    }
}
