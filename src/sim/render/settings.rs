//! Rendering settings.

use crate::{clone, fmt_report};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
    /// Optional limit on number of threads to use.
    num_threads: Option<usize>,
    /// Number of tracers to simulate in each thread block.
    block_size: usize,
    /// Bump distance (m).
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Minimum statistical weight to consider.
    min_weight: f64,
}

impl Settings {
    clone!(num_threads, Option<usize>);
    clone!(block_size, usize);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    clone!(min_weight, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        num_threads: Option<usize>,
        block_size: usize,
        bump_dist: f64,
        loop_limit: u64,
        min_weight: f64,
    ) -> Self {
        debug_assert!(num_threads.is_none() || num_threads.unwrap() >= 1);
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);
        debug_assert!(min_weight >= 0.0);

        Self {
            num_threads,
            block_size,
            bump_dist,
            loop_limit,
            min_weight,
        }
    }
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        if let Some(num_threads) = self.num_threads {
            fmt_report!(fmt, num_threads, "num_threads");
        } else {
            fmt_report!(fmt, "unlimited", "num_threads");
        }
        fmt_report!(fmt, self.block_size, "block size");
        fmt_report!(fmt, self.bump_dist, "bump distance");
        fmt_report!(fmt, self.loop_limit, "loop limit");
        fmt_report!(fmt, self.min_weight, "minimum weight");
        Ok(())
    }
}
