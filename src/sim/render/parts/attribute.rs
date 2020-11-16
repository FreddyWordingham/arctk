//! Attributes implementation.

use crate::img::Gradient;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Opaque coloured surface.
    Opaque(&'a Gradient),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}
