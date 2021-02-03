//! Attribute first-stage linker.

use crate::{
    err::Error,
    ord::{Link, Name, Set},
    sim::mcrt::{AttributeLinker, Detector},
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Surface attribute setup.
#[file]
pub enum AttributeLinkerLinker {
    /// Material interface, inside material name, outside material name.
    Interface(Name, Name),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
    /// Spectrometer.
    Detector(Name),
}

impl<'a> Link<'a, Detector> for AttributeLinkerLinker {
    type Inst = AttributeLinker<'a>;

    #[inline]
    fn requires(&self) -> Vec<Name> {
        match *self {
            Self::Interface(..) | Self::Mirror(..) => vec![],
            Self::Detector(ref name) => vec![name.clone()],
        }
    }

    #[inline]
    fn link(self, detectors: &'a Set<Detector>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(inside, outside) => Self::Inst::Interface(inside, outside),
            Self::Mirror(r) => Self::Inst::Mirror(r),
            Self::Detector(ref name) => Self::Inst::Detector(
                detectors
                    .get(name)
                    .unwrap_or_else(|| panic!("Failed to link attribute-detector key: {}", name)),
            ),
        })
    }
}

impl Display for AttributeLinkerLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Self::Interface(ref in_mat, ref out_mat) => {
                write!(fmt, "Interface: {} :| {}", in_mat, out_mat)
            }
            Self::Mirror(abs) => {
                write!(fmt, "Mirror: {}% abs", abs * 100.0)
            }
            Self::Detector(ref name) => {
                write!(fmt, "Detector: {}", name)
            }
        }
    }
}
