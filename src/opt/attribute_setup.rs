//! Attributes implementation.

use crate::{
    opt::{Attribute, Material},
    ord::Set,
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

impl AttributeSetup {
    /// Setup the attribute links.
    #[must_use]
    #[inline]
    fn setup(self, mats: &Set<Material>) -> Attribute {
        match self {
            Self::Mirror(abs) => Attribute::Mirror(abs),
            Self::Refractive(ref inside, ref outside) => {
                Attribute::Refractive(&mats[inside], &mats[outside])
            }
        }
    }
}
