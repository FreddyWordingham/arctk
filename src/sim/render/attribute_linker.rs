//! Visual attributes.

use crate::{
    img::Gradient,
    ord::{Link, Name, Set},
    sim::render::Attribute,
};
use arctk_attr::file;
use std::fmt::{Display, Error, Formatter};

/// Surface attribute setup.
#[file]
pub enum AttributeLinker {
    /// Opaque coloured surface.
    Opaque(Name),
    /// Partially reflective mirror, absorption fraction.
    Mirror(Name, f64),
    /// Partially transparent, absorption fraction.
    Transparent(Name, f64),
    /// Refractive, absorption fraction, inside and outside refractive indices.
    Refractive(Name, f64, [f64; 2]),
    /// Luminous surface, brightness multiplier.
    Luminous(Name, f64),
}

impl Display for AttributeLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match *self {
            Self::Opaque(ref grad) => {
                write!(fmt, "Opaque: [{}]", grad)
            }
            Self::Mirror(ref grad, abs_frac) => {
                write!(fmt, "Mirror: {}% [{}]", abs_frac * 100.0, grad)
            }
            Self::Transparent(ref grad, abs_frac) => {
                write!(fmt, "Transparent: {}% [{}]", abs_frac * 100.0, grad)
            }
            Self::Refractive(ref grad, abs_frac, [inside, outside]) => {
                write!(
                    fmt,
                    "Transparent: {}% {}:|{} [{}]",
                    abs_frac * 100.0,
                    inside,
                    outside,
                    grad,
                )
            }
            Self::Luminous(ref grad, multiplier) => {
                write!(fmt, "Luminous: {}% [{}]", multiplier * 100.0, grad)
            }
        }
    }
}
