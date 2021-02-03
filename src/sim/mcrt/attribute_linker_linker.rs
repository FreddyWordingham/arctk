//! Attribute first-stage linker.

use crate::{
    err::Error,
    ord::{Link, Name, Set},
    sim::mcrt::AttributeLinker,
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
    Spectrometer(Name),
}

impl<'a> Link<'a, usize> for AttributeLinkerLinker {
    type Inst = AttributeLinker;

    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        if let Self::Spectrometer(name) = self {
            vec![name.clone()]
        } else {
            vec![]
        }
    }

    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        Ok(match self {
            Self::Interface(inside, outside) => Self::Inst::Interface(inside, outside),
            Self::Mirror(r) => Self::Inst::Mirror(r),
            Self::Spectrometer(ref name) => {
                Self::Inst::Spectrometer(*reg.get(&name).unwrap_or_else(|| {
                    panic!(
                        "Failed to link attribute-linker-spectrometer-index: {}",
                        name
                    )
                }))
            }
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
            Self::Spectrometer(ref id) => {
                write!(fmt, "Spectrometer: {}", id)
            }
        }
    }
}
