//! Attributes implementation.

use crate::img::Gradient;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Opaque coloured surface.
    Opaque(&'a Gradient),
    /// Partially reflective mirror, absorption fraction.
    Mirror(&'a Gradient, f64),
    /// Partially transparent, absorption fraction.
    Transparent(&'a Gradient, f64),
}
