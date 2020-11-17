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
    /// Material interface, inside material name, outside material name.
    Interface(String, String),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}

impl<'a> Link<'a, Material> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<String> {
        match *self {
            Self::Interface(ref inside, ref outside) => vec![inside.clone(), outside.clone()],
            Self::Mirror(..) => vec![],
        }
    }

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(ref inside, ref outside) => {
                Attribute::Interface(&mats[inside], &mats[outside])
            }
            Self::Mirror(r) => Attribute::Mirror(r),
        })
    }
}
