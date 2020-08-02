//! RGB implementation.

/// RGB format channel indexing.
pub enum RGB {
    /// Red channel.
    Red,
    /// Green channel.
    Green,
    /// Blue channel.
    Blue,
}

/// Red convenience constant.
pub const RED: usize = RGB::Red as usize;

/// Green convenience constant.
pub const GREEN: usize = RGB::Green as usize;

/// Blue convenience constant.
pub const BLUE: usize = RGB::Blue as usize;
