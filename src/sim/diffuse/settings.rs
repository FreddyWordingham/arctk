//! Diffuse settings.

use crate::{clone, fmt_report};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
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
    clone!(time, f64);
    clone!(dumps, usize);
    clone!(quality, f64);
    clone!(block_size, usize);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(time: f64, dumps: usize, quality: f64, block_size: usize) -> Self {
        debug_assert!(time > 0.0);
        debug_assert!(dumps > 0);
        debug_assert!(quality > 0.0);
        debug_assert!(quality < 1.0);
        debug_assert!(block_size > 0);

        Self {
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
        fmt_report!(fmt, self.time, "integration time (s)");
        fmt_report!(fmt, self.dumps, "intermediate dumps");
        fmt_report!(fmt, self.quality, "quality parameter");
        fmt_report!(fmt, self.block_size, "block size");
        Ok(())
    }
}
