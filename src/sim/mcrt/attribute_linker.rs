//! Attribute second-stage linker.

use crate::{
    err::Error,
    ord::{Link, Name, Set},
    phys::Material,
    sim::mcrt::{Attribute, Detector},
};
use std::fmt::{Display, Formatter};

/// Surface attribute setup.
pub enum AttributeLinker<'a> {
    /// Material interface, inside material name, outside material name.
    Interface(Name, Name),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer.
    Detector(&'a Detector),
}

impl<'a> Link<'a, Material> for AttributeLinker<'a> {
    type Inst = Attribute<'a>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        match *self {
            Self::Interface(ref inside, ref outside) => vec![inside.clone(), outside.clone()],
            Self::Mirror(..) | Self::Detector(..) => vec![],
        }
    }

    #[inline]
    fn link(self, mats: &'a Set<Material>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(ref inside, ref outside) => Self::Inst::Interface(
                mats.get(inside).unwrap_or_else(|| {
                    panic!("Failed to link attribute-interface key: {}", inside)
                }),
                mats.get(outside).unwrap_or_else(|| {
                    panic!("Failed to link attribute-interface key: {}", outside)
                }),
            ),
            Self::Mirror(r) => Self::Inst::Mirror(r),
            Self::Detector(det) => Self::Inst::Detector(det),
        })
    }
}

impl<'a> Display for AttributeLinker<'a> {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Interface(ref in_mat, ref out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Detector(detector) => {
                write!(fmt, "Detector: {}", detector)
            }
        }
    }
}
