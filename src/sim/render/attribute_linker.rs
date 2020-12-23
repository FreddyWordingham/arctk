//! Visual attributes.

use crate::{
    err::Error,
    img::Gradient,
    ord::{Link, Name, Set},
    sim::render::Attribute,
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

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

impl<'a> Link<'a, Gradient> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        match *self {
            Self::Opaque(ref grad)
            | Self::Mirror(ref grad, ..)
            | Self::Transparent(ref grad, ..)
            | Self::Refractive(ref grad, ..)
            | Self::Luminous(ref grad, ..) => vec![grad.clone()],
        }
    }

    #[inline]
    fn link(self, grads: &'a Set<Gradient>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Opaque(ref grad) => Attribute::Opaque(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
            ),
            Self::Mirror(ref grad, abs_frac) => Attribute::Mirror(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                abs_frac,
            ),
            Self::Transparent(ref grad, abs_frac) => Attribute::Transparent(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                abs_frac,
            ),
            Self::Refractive(ref grad, abs_frac, ref_indices) => Attribute::Refractive(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                abs_frac,
                ref_indices,
            ),
            Self::Luminous(ref grad, bright_mult) => Attribute::Luminous(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                bright_mult,
            ),
        })
    }
}

impl Display for AttributeLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
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
