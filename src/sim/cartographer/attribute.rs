//! Optical attributes.

use std::fmt::{Display, Error, Formatter};

/// Surface attributes.
pub enum Attribute {
    /// Material interface, inside material index, outside material index.
    Interface(usize, usize),
    /// Partially reflective mirror, reflection fraction.
    Mirror(f64),
}

impl Display for Attribute {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
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
