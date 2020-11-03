//! Rate building structure.

use crate::{
    chem::Rate,
    err::Error,
    ord::{Name, Register},
};
use arctk_attr::load;

/// Rate of reaction builder.
#[load]
pub struct RateBuilder(f64, Vec<(String, f64)>);

impl RateBuilder {
    /// Get a list of all species involved in the rate calculation.
    #[inline]
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for (n, _) in &self.1 {
            names.push(n.to_string());
        }

        names.sort();
        names.dedup();

        names
    }
}

impl Name for RateBuilder {
    type Inst = Rate;

    #[inline]
    fn reg(self, reg: &Register) -> Result<Self::Inst, Error> {
        let mut orders = Vec::with_capacity(self.1.len());
        for (name, m) in self.1 {
            orders.push((reg.index(&name), m))
        }

        Ok(Self::Inst::new(self.0, orders))
    }
}
