//! Greek alphabet.

/// Greek letters.
pub enum Greek {
    /// First letter.
    Alpha,
    /// Second letter.
    Beta,
    /// Third letter.
    Gamma,
}

/// Greek Alpha convenience indexing constant.
pub const ALPHA: usize = Greek::Alpha as usize;

/// Greek Beta convenience indexing constant.
pub const BETA: usize = Greek::Beta as usize;

/// Greek Gamma convenience indexing constant.
pub const GAMMA: usize = Greek::Gamma as usize;
