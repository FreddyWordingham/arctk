//! Red-Green-Blue colour representation.

/// RGB format.
pub enum RGB {
    /// Red channel.
    Red,
    /// Green channel.
    Green,
    /// Blue channel.
    Blue,
}

/// Red convenience indexing constant.
pub const RED: usize = RGB::Red as usize;

/// Green convenience indexing constant.
pub const GREEN: usize = RGB::Green as usize;

/// Blue convenience indexing constant.
pub const BLUE: usize = RGB::Blue as usize;
