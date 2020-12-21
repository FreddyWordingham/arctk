//! Attribute linker.

use crate::{
    err::Error,
    ord::{Link, Name, Set},
    phys::{Attribute, Material},
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Surface attribute setup.
#[file]
pub enum AttributeLinker {
    /// Material interface, inside material name, outside material name.
    Interface(Name, Name),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}

impl<'a> Link<'a, Material> for AttributeLinker {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        match *self {
            Self::Interface(ref inside, ref outside) => vec![inside.clone(), outside.clone()],
            Self::Mirror(..) => vec![],
        }
    }

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(ref inside, ref outside) => Attribute::Interface(
                mats.get(inside).unwrap_or_else(|| {
                    panic!("Failed to link attribute-interface key: {}", inside)
                }),
                mats.get(outside).unwrap_or_else(|| {
                    panic!("Failed to link attribute-interface key: {}", outside)
                }),
            ),
            Self::Mirror(r) => Attribute::Mirror(r),
        })
    }
}

impl Register for AttributeLinker {
    type Inst = AttributeIdx;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        match *self {
            Self::Interface(ref inside, ref outside) => vec![inside.clone(), outside.clone()],
            Self::Mirror(..) => vec![],
        }
    }

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(ref inside, ref outside) => Attribute::Interface(
                mats.get(inside).unwrap_or_else(|| {
                    panic!("Failed to link attribute-interface key: {}", inside)
                }),
                mats.get(outside).unwrap_or_else(|| {
                    panic!("Failed to link attribute-interface key: {}", outside)
                }),
            ),
            Self::Mirror(r) => Attribute::Mirror(r),
        })
    }
}

impl Display for AttributeLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Interface(ref in_mat, ref out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
        }
    }
}
