//! Greek implementation.

/// Greek letter indexing.
pub enum Greek {
    /// First letter.
    Alpha,
    /// Second letter.
    Beta,
    /// Third letter.
    Gamma,
}

/// Greek Alpha convenience constant.
pub const ALPHA: usize = Greek::Alpha as usize;

/// Greek Beta convenience constant.
pub const BETA: usize = Greek::Beta as usize;

/// Greek Gamma convenience constant.
pub const GAMMA: usize = Greek::Gamma as usize;
