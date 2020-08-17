//! Reaction builder implementation.

use crate::{
    kinetics::{Name, RateBuilder, Reaction, Register},
    Error, Group,
};
use attr::load;

/// Loadable reaction structure.
#[load]
pub struct ReactionBuilder {
    /// Reaction reactants and their respective stoichiometric coefficients.
    reactants: Vec<(i32, Group)>,
    /// Reaction products and their respective stoichiometric coefficients.
    products: Vec<(i32, Group)>,
    /// Reaction rate.
    rate: RateBuilder,
}

impl ReactionBuilder {
    /// Get a list of all species involved in the reaction.
    #[inline]
    #[must_use]
    pub fn names(&self) -> Vec<Group> {
        let mut names = Vec::new();
        for (_, n) in &self.reactants {
            names.push(n.to_string());
        }
        for (_, n) in &self.products {
            names.push(n.to_string());
        }
        names.append(&mut self.rate.names());

        names.sort();
        names.dedup();

        names
    }
}

impl Name for ReactionBuilder {
    type Inst = Reaction;

    #[inline]
    fn build(self, reg: &Register) -> Result<Self::Inst, Error> {
        let mut reacts = Vec::with_capacity(self.reactants.len());
        for (s, name) in self.reactants {
            reacts.push((s, reg.index(&name)));
        }

        let mut prods = Vec::with_capacity(self.products.len());
        for (s, name) in self.products {
            prods.push((s, reg.index(&name)));
        }

        Ok(Self::Inst::new(
            reacts,
            prods,
            self.rate.build(reg)?,
            reg.names().len(),
        ))
    }
}
