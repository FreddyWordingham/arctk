//! Red-Green-Blue colour representation.

/// RGB format.
pub enum Rgb {
    /// Red channel.
    Red,
    /// Green channel.
    Green,
    /// Blue channel.
    Blue,
}

/// Red convenience indexing constant.
pub const RED: usize = Rgb::Red as usize;

/// Green convenience indexing constant.
pub const GREEN: usize = Rgb::Green as usize;

/// Blue convenience indexing constant.
pub const BLUE: usize = Rgb::Blue as usize;
