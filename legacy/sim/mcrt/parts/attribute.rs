//! Attributes implementation.

use crate::sim::mcrt::Material;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Material interface, inside material, outside material.
    Interface(&'a Material, &'a Material),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}
