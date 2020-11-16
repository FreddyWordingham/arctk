//! Attributes implementation.

use crate::sim::mcrt::Material;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Partially reflective mirror, absorption fraction.
    Mirror(f64),
    /// Material interface, inside material, outside material.
    Interface(&'a Material, &'a Material),
}
