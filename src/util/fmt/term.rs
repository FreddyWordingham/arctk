//! Terminal width calculation function.

use terminal_size::terminal_size;

/// Determine the terminal width.
/// Return the fallback size if the width could not be determined.
/// # Errors
/// if the terminal width can not be determined.
#[inline]
pub fn width() -> Result<usize, String> {
    let ts = terminal_size().ok_or("Failed to determine terminal width.")?;

    Ok((ts.0).0 as usize)
}
