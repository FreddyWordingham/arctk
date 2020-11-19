//! Attributes implementation.

use crate::{
    err::Error,
    img::Gradient,
    ord::{Link, Set},
    sim::render::Attribute,
};
use arctk_attr::load;

/// Surface attribute setup.
#[load]
pub enum AttributeLinker {
    /// Opaque coloured surface.
    Opaque(String),
    /// Partially reflective mirror, reflection fraction.
    Mirror(String, f64),
    /// Partially transparent, transmission fraction.
    Transparent(String, f64),
}

impl<'a> Link<'a, Gradient> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<String> {
        match *self {
            Self::Opaque(ref grad) => vec![grad.clone()],
            Self::Mirror(ref grad, ..) => vec![grad.clone()],
            Self::Transparent(ref grad, ..) => vec![grad.clone()],
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
            Self::Mirror(ref grad, ref_frac) => Attribute::Mirror(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                ref_frac,
            ),
            Self::Transparent(ref grad, trans_frac) => Attribute::Transparent(
                grads
                    .get(grad)
                    .unwrap_or_else(|| panic!("Failed to link attribute-gradient key: {}", grad)),
                trans_frac,
            ),
        })
    }
}
