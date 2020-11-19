//! Attributes implementation.

use crate::img::Gradient;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Opaque coloured surface.
    Opaque(&'a Gradient),
    /// Partially reflective mirror, reflection fraction.
    Mirror(&'a Gradient, f64),
    /// Partially transparent, transmission fraction.
    Transparent(&'a Gradient, f64),
}
