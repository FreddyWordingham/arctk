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
    Mirror(f64),
}

impl<'a> Link<'a, Gradient> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<String> {
        match *self {
            Self::Opaque(ref grad) => vec![grad.clone()],
            Self::Mirror(..) => vec![],
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
            Self::Mirror(ref_frac) => Attribute::Mirror(ref_frac),
        })
    }
}
