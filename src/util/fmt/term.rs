//! Terminal width calculation function.

use terminal_size::terminal_size;

/// Determine the terminal width.
/// Return the fallback size if the width could not be determined.
/// # Errors
/// if the terminal width can not be determined.
#[inline]
pub fn width(default: usize) -> usize {
    if let Some((width, _)) = terminal_size() {
        width.0 as usize
    } else {
        default
    }
}
