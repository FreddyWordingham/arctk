//! Attributes implementation.

use crate::{
    err::Error,
    ord::{Link, Set},
    sim::mcrt::{Attribute, Material},
};
use arctk_attr::load;

/// Surface attribute setup.
#[load]
pub enum AttributeLinker {
    /// Partially reflective mirror, absorption fraction.
    Mirror(f64),
    /// Material interface, inside material name, outside material name.
    Interface(String, String),
}

impl<'a> Link<'a, Material> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Mirror(abs) => Attribute::Mirror(abs),
            Self::Interface(ref inside, ref outside) => {
                Attribute::Interface(&mats[inside], &mats[outside])
            }
        })
    }
}
