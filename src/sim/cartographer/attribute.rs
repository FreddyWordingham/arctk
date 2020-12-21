//! Optical attributes.

/// Surface attributes.
pub enum Attribute {
    /// Material interface, inside material index, outside material index.
    Interface(usize, usize),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}
