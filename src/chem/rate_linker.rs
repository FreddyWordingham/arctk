//! Rate building structure.

use crate::{
    chem::Rate,
    err::Error,
    ord::{Link, Name, Set},
};
use arctk_attr::load;

/// Rate of reaction builder.
#[load]
pub struct RateLinker(f64, Vec<(Name, f64)>);

impl<'a> Link<'a, usize> for RateLinker {
    type Inst = Rate;

    /// Get a list of all required resource keys.
    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        let mut names = Vec::with_capacity(self.1.len());
        for (name, _x) in &self.1 {
            names.push(name.clone());
        }

        names
    }

    /// Link the instance type.
    /// # Errors
    /// if a field could not be referenced.
    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        let mut orders = Vec::with_capacity(self.1.len());
        for (name, m) in self.1 {
            orders.push((
                *reg.get(&name)
                    .unwrap_or_else(|| panic!("Failed to link rate-index: {}", name)),
                m,
            ))
        }

        Ok(Rate::new(self.0, orders))
    }
}
