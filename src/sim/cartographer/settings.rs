//! Cartographer settings.

use crate::{
    access, clone, fmt_report,
    sim::cartographer::{Caster, SuperSample},
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
    /// Optional limit on number of threads to use.
    num_threads: Option<usize>,
    /// Number of photons to simulate in each thread block.
    block_size: usize,
    /// Bump distance (m).
    bump_dist: f64,
    /// Loop limit.
    loop_limit: u64,
    /// Caster settings.
    caster: Caster,
    /// Super sampling.
    super_sampling: SuperSample,
}

impl Settings {
    clone!(num_threads, Option<usize>);
    clone!(block_size, usize);
    clone!(bump_dist, f64);
    clone!(loop_limit, u64);
    access!(caster, Caster);
    access!(super_sampling, SuperSample);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        num_threads: Option<usize>,
        block_size: usize,
        bump_dist: f64,
        loop_limit: u64,
        caster: Caster,
        super_sampling: SuperSample,
    ) -> Self {
        debug_assert!(num_threads.is_none() || num_threads.unwrap() >= 1);
        debug_assert!(block_size > 0);
        debug_assert!(bump_dist > 0.0);

        Self {
            num_threads,
            block_size,
            bump_dist,
            loop_limit,
            caster,
            super_sampling,
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
        fmt_report!(fmt, self.bump_dist, "bump distance [m]");
        fmt_report!(fmt, self.loop_limit, "loop limit");
        fmt_report!(fmt, self.caster, "caster");
        fmt_report!(fmt, self.super_sampling, "super sampling");
        Ok(())
    }
}
