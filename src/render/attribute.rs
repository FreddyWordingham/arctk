//! Observable attributes.

use palette::{Gradient, LinSrgba};

/// Observable attributes.
#[non_exhaustive]
pub enum Attribute<'a> {
    /// Opaque coloured surface.
    Opaque(&'a Gradient<LinSrgba>),
    /// Partially reflective mirror, absorption fraction.
    Mirror(&'a Gradient<LinSrgba>, f64),
    /// Partially transparent, absorption fraction.
    Transparent(&'a Gradient<LinSrgba>, f64),
    /// Refractive, absorption fraction, inside and outside refractive indices.
    Refractive(&'a Gradient<LinSrgba>, f64, [f64; 2]),
    /// Luminous surface, brightness multiplier.
    Luminous(&'a Gradient<LinSrgba>, f64),
    /// Switchable condition, conditional value.
    Switchable([&'a Gradient<LinSrgba>; 2], f64),
}
