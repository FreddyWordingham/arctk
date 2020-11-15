//! Attributes implementation.

/// Surface attributes.
pub enum Attribute {
    /// Partially reflective mirror, absorption fraction.
    Mirror(f64),
    /// Refractive interface, inside material, outside material.
    Refractive(usize, usize),
}
