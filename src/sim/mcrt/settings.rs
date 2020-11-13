//! MCRT settings.

use crate::{access, clone, math::Pos3, ord::Key};
use arctk_attr::load;

/// MCRT settings structure.
#[load]
pub struct Settings {
    /// Number of photons to simulate in each thread block.
    block_size: u64,
    /// Number of photons to simulate.
    num_phot: u64,
    /// Bump distance [m].
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Weight to perform roulette at.
    roulette_weight: f64,
    /// Number of roulette barrels.
    roulette_barrels: u64,
    /// Initial emission material.
    init_mat: Key,
    /// Peel-off detector position.
    detector_pos: Pos3,
}

impl Settings {
    clone!(block_size, u64);
    clone!(num_phot, u64);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    clone!(roulette_weight, f64);
    clone!(roulette_barrels, u64);
    access!(init_mat, Key);
    access!(detector_pos, Pos3);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        block_size: u64,
        num_phot: u64,
        bump_dist: f64,
        loop_limit: u64,
        roulette_weight: f64,
        roulette_barrels: u64,
        init_mat: Key,
        detector_pos: Pos3,
    ) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(num_phot > 0);
        debug_assert!(bump_dist > 0.0);
        debug_assert!(loop_limit > 0);
        debug_assert!(roulette_weight >= 0.0);
        debug_assert!(roulette_barrels > 0);

        Self {
            block_size,
            num_phot,
            bump_dist,
            loop_limit,
            roulette_weight,
            roulette_barrels,
            init_mat,
            detector_pos,
        }
    }
}
