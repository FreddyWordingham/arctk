//! Attributes implementation.

use crate::ord::{Register, Setup};
use crate::sim::mcrt::Attribute;
use arctk_attr::load;

/// Surface attribute setup.
#[load]
pub enum AttributeSetup {
    /// Partially reflective mirror, absorption fraction.
    Mirror(f64),
    /// Refractive interface, inside material, outside material.
    Refractive(String, String),
}

impl Setup for AttributeSetup {
    type Inst = Attribute;

    #[inline]
    fn setup(self, reg: &Register) -> Self::Inst {
        match self {
            Self::Mirror(abs) => Self::Inst::Mirror(abs),
            Self::Refractive(ref inside, ref outside) => {
                Self::Inst::Refractive(reg.index_of(inside), reg.index_of(outside))
            }
        }
    }
}
