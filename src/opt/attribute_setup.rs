//! Attributes implementation.

use crate::{
    err::Error,
    opt::{Attribute, Material},
    ord::{Set, Setup},
};
use arctk_attr::load;

/// Surface attribute setup.
#[load]
pub enum AttributeSetup {
    /// Partially reflective mirror, absorption fraction.
    Mirror(f64),
    /// Refractive interface, inside material name, outside material name.
    Refractive(String, String),
}

impl<'a> Setup<'a, Material> for AttributeSetup {
    type Inst = Attribute<'a>;

    #[inline]
    fn setup(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Mirror(abs) => Attribute::Mirror(abs),
            Self::Refractive(ref inside, ref outside) => {
                Attribute::Refractive(&mats[inside], &mats[outside])
            }
        })
    }
}
