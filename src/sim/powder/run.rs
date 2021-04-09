//! Simulation control functions.

use crate::{err::Error, sim::powder::Input};

/// Run a single-threaded powder simulation.
/// # Errors
/// if the progress bar can not be locked.
#[allow(clippy::expect_used)]
#[inline]
pub fn single_thread<'a>(input: &'a Input) -> Result<(), Error> {
    Ok(())
}
