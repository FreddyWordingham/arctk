//! Terminal width calculation function.

use terminal_size::terminal_size;

/// Fallback terminal width if it cannot be determined at runtime.
const FALLBACK_TERM_WIDTH: usize = 80;

/// Determine the terminal width.
/// Return the fallback size if the width could not be determined.
#[inline]
#[must_use]
pub fn width() -> usize {
    let ts = terminal_size();

    if let Some(ts) = ts {
        (ts.0).0 as usize
    } else {
        FALLBACK_TERM_WIDTH
    }
}
