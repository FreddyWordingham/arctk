//! MCRT settings.

use crate::{clone, fmt_report};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
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
}

impl Settings {
    clone!(num_phot, usize);
    clone!(block_size, usize);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    clone!(min_weight, f64);
    clone!(roulette_barrels, u64);

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
        }
    }
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.num_phot, "num_phot");
        fmt_report!(fmt, self.block_size, "block size");
        fmt_report!(fmt, self.bump_dist, "bump distance (m)");
        fmt_report!(fmt, self.loop_limit, "loop limit");
        fmt_report!(fmt, self.min_weight, "minimum simulation weight");
        fmt_report!(fmt, self.roulette_barrels, "roulette barrels");
        Ok(())
    }
}
