//! Array linker structure.

use crate::{
    err::Error,
    ord::{Link, Name, Set},
};
use arctk_attr::file;
use ndarray::Array1;
use std::fmt::{Display, Formatter};

/// Array linking structure.
#[file]
pub struct ArrayLinker(Vec<(Name, f64)>);

impl ArrayLinker {
    /// Get the length of the wrapped vec.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Determine if the wrapped vec is unoccupied.
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a> Link<'a, usize> for ArrayLinker {
    type Inst = Array1<f64>;

    /// Get a list of all required resource keys.
    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        let mut names = Vec::with_capacity(self.0.len());
        for &(ref name, ref _x) in &self.0 {
            names.push(name.clone());
        }

        names
    }

    /// Link the instance type.
    /// # Errors
    /// if a field could not be referenced.
    #[inline]
    fn link(self, reg: &'a mut Set<usize>) -> Result<Self::Inst, Error> {
        let mut arr = Array1::zeros(reg.len());
        for (name, x) in self.0 {
            arr[*reg
                .get(&name)
                .unwrap_or_else(|| panic!("Failed to link named-index: {}", name))] = x;
        }

        Ok(arr)
    }
}

impl Display for ArrayLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        if !self.0.is_empty() {
            write!(fmt, "{}{}", self.0[0].1, self.0[0].0)?;

            for &(ref name, value) in self.0.iter().skip(1) {
                write!(fmt, " {}{}", value, name)?;
            }
        }

        Ok(())
    }
}
