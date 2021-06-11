//! Reactor settings.

use crate::{clone, fmt_report};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
    /// Optional limit on number of threads to use.
    num_threads: Option<usize>,
    /// Total integration time (s).
    time: f64,
    /// Number of intermediate dumps.
    dumps: usize,
    /// Diffusion quality parameter \[0:1\].
    d_quality: f64,
    /// Reaction quality parameter \[0:1\].
    r_quality: f64,
    /// Minimum timestep (s).
    min_time: f64,
    /// Diffusion block size.
    d_block_size: usize,
    /// Reaction block size.
    r_block_size: usize,
}

impl Settings {
    clone!(num_threads, Option<usize>);
    clone!(time, f64);
    clone!(dumps, usize);
    clone!(d_quality, f64);
    clone!(r_quality, f64);
    clone!(min_time, f64);
    clone!(d_block_size, usize);
    clone!(r_block_size, usize);

    /// Construct a new instance.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    #[must_use]
    pub fn new(
        num_threads: Option<usize>,
        time: f64,
        dumps: usize,
        d_quality: f64,
        r_quality: f64,
        min_time: f64,
        d_block_size: usize,
        r_block_size: usize,
    ) -> Self {
        debug_assert!(num_threads.is_none() || num_threads.unwrap() >= 1);
        debug_assert!(time > 0.0);
        debug_assert!(dumps > 0);
        debug_assert!(d_quality > 0.0);
        debug_assert!(d_quality < 1.0);
        debug_assert!(r_quality > 0.0);
        debug_assert!(r_quality < 1.0);
        debug_assert!(min_time < time);
        debug_assert!(d_block_size > 0);
        debug_assert!(r_block_size > 0);

        Self {
            num_threads,
            time,
            dumps,
            d_quality,
            r_quality,
            min_time,
            d_block_size,
            r_block_size,
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
        fmt_report!(fmt, self.time, "integration time (s)");
        fmt_report!(fmt, self.dumps, "intermediate dumps");
        fmt_report!(fmt, self.d_quality, "diffusion quality parameter");
        fmt_report!(fmt, self.r_quality, "reaction quality parameter");
        fmt_report!(fmt, self.min_time, "minimum timestep (s)");
        fmt_report!(fmt, self.d_block_size, "diffusion block size");
        fmt_report!(fmt, self.r_block_size, "reaction block size");
        Ok(())
    }
}
