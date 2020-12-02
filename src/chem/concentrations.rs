//! Concentration builder structure.

use crate::{
    err::Error,
    ord::{Link, Name, Set},
};
use ndarray::Array1;

/// Loadable concentration structure.
pub type Concentrations = Vec<(Name, f64)>;

impl<'a> Link<'a, usize> for Concentrations {
    type Inst = Array1<f64>;

    /// Get a list of all required resource keys.
    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        let mut names = Vec::with_capacity(self.len());
        for (name, _x) in self {
            names.push(name.clone());
        }

        names
    }

    /// Link the instance type.
    /// # Errors
    /// if a field could not be referenced.
    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        let mut arr = Array1::zeros(reg.len());
        for (name, x) in self {
            arr[*reg
                .get(&name)
                .unwrap_or_else(|| panic!("Failed to link concentration-index: {}", name))] = x;
        }

        Ok(arr)
    }
}
