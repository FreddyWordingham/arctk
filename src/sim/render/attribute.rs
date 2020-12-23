//! Attributes implementation.

use crate::{img::Gradient, util::fmt::gradient::to_string};
use std::fmt::{Display, Error, Formatter};

/// Surface attributes.
pub enum Attribute<'a> {
    /// Opaque coloured surface.
    Opaque(&'a Gradient),
    /// Partially reflective mirror, absorption fraction.
    Mirror(&'a Gradient, f64),
    /// Partially transparent, absorption fraction.
    Transparent(&'a Gradient, f64),
    /// Refractive, absorption fraction, inside and outside refractive indices.
    Refractive(&'a Gradient, f64, [f64; 2]),
    /// Luminous surface, brightness multiplier.
    Luminous(&'a Gradient, f64),
}

impl Display for Attribute<'_> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Opaque(grad) => {
                write!(fmt, "Opaque: [{}]", to_string(grad, 32))
            }
            Self::Mirror(grad, abs_frac) => {
                write!(
                    fmt,
                    "Mirror: {}% [{}]",
                    abs_frac * 100.0,
                    to_string(grad, 32)
                )
            }
            Self::Transparent(grad, abs_frac) => {
                write!(
                    fmt,
                    "Transparent: {}% [{}]",
                    abs_frac * 100.0,
                    to_string(grad, 32)
                )
            }
            Self::Refractive(grad, abs_frac, [inside, outside]) => {
                write!(
                    fmt,
                    "Transparent: {}% {}:|{} [{}]",
                    abs_frac * 100.0,
                    inside,
                    outside,
                    to_string(grad, 32),
                )
            }
            Self::Luminous(grad, multiplier) => {
                write!(
                    fmt,
                    "Luminous: {}% [{}]",
                    multiplier * 100.0,
                    to_string(grad, 32)
                )
            }
        }
    }
}
