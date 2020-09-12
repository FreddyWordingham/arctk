//! Rate builder implementation.

use crate::{
    kinetics::{Name, Rate, Register},
    Error, Group,
};
use arctk_attr::load;

/// Loadable rate structure.
#[load]
pub struct RateBuilder(f64, Vec<(Group, f64)>);

impl RateBuilder {
    /// Get a list of all species involved in the rate calculation.
    #[inline]
    #[must_use]
    pub fn names(&self) -> Vec<Group> {
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
    fn build(self, reg: &Register) -> Result<Self::Inst, Error> {
        let mut orders = Vec::with_capacity(self.1.len());
        for (name, m) in self.1 {
            orders.push((reg.index(&name), m))
        }

        Ok(Self::Inst::new(self.0, orders))
    }
}
