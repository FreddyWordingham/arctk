//! Optical attributes.

use crate::phys::Material;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Material interface, inside material, outside material.
    Interface(&'a Material, &'a Material),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}
