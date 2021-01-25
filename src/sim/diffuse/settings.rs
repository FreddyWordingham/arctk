//! Diffuse settings.

use crate::{clone, fmt_report};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// General settings structure.
#[file]
pub struct Settings {
    /// Number of cells to simulate in each thread block.
    block_size: usize,
    /// Total integration time [s].
    time: f64,
    /// Number of intermediate dumps.
    dumps: usize,
    /// Accuracy parameter [0:1].
    accuracy: f64,
}

impl Settings {
    clone!(block_size, usize);
    clone!(time, f64);
    clone!(dumps, usize);
    clone!(accuracy, f64);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(block_size: usize, time: f64, dumps: usize, accuracy: f64) -> Self {
        debug_assert!(block_size > 0);
        debug_assert!(time > 0.0);
        debug_assert!(dumps > 0);
        debug_assert!(accuracy > 0.0);
        debug_assert!(accuracy <= 1.0);

        Self {
            block_size,
            time,
            dumps,
            accuracy,
        }
    }
}

impl Display for Settings {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        writeln!(fmt, "...")?;
        fmt_report!(fmt, self.block_size, "block size");
        fmt_report!(fmt, self.time, "integration time (s)");
        fmt_report!(fmt, self.dumps, "intermediate dumps");
        fmt_report!(fmt, self.accuracy, "accuracy parameter");
        Ok(())
    }
}
