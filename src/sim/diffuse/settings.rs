//! Diffuse settings.

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
    /// Quality parameter \[0:1\].
    quality: f64,
    /// Number of photons to simulate in each thread block.
    block_size: usize,
}

impl Settings {
    clone!(num_threads, Option<usize>);
    clone!(time, f64);
    clone!(dumps, usize);
    clone!(quality, f64);
    clone!(block_size, usize);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(
        num_threads: Option<usize>,
        time: f64,
        dumps: usize,
        quality: f64,
        block_size: usize,
    ) -> Self {
        debug_assert!(num_threads.is_none() || num_threads.unwrap() >= 1);
        debug_assert!(time > 0.0);
        debug_assert!(dumps > 0);
        debug_assert!(quality > 0.0);
        debug_assert!(quality < 1.0);
        debug_assert!(block_size > 0);

        Self {
            num_threads,
            time,
            dumps,
            quality,
            block_size,
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
        fmt_report!(fmt, self.quality, "quality parameter");
        fmt_report!(fmt, self.block_size, "block size");
        Ok(())
    }
}
