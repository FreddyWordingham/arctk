//! Attributes implementation.

use crate::opt::Material;

/// Surface attributes.
pub enum Attribute<'a> {
    /// Partially reflective mirror, absorption fraction.
    Mirror(f64),
    /// Refractive interface, inside material index, outside material index.
    Refractive(&'a Material, &'a Material),
}
