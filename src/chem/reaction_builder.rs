//! Reaction building structure.

use crate::{
    chem::{RateBuilder, Reaction},
    err::Error,
    ord::{Name, Register},
};
use arctk_attr::load;
use ndarray::Array1;

/// Reaction builder.
#[load]
pub struct ReactionBuilder {
    /// Reaction rate.
    rate: RateBuilder,
    /// List of each reactant species and its stoichiometric coefficient.
    reactants: Vec<(String, f64)>,
    /// List of each product species and its stoichiometric coefficient.
    products: Vec<(String, f64)>,
}

impl ReactionBuilder {
    /// Get a list of all species involved in the reaction.
    #[inline]
    #[must_use]
    pub fn names(&self) -> Vec<String> {
        let mut names = Vec::new();
        for (n, _) in &self.reactants {
            names.push(n.to_string());
        }
        for (n, _) in &self.products {
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
    fn reg(self, reg: &Register) -> Result<Self::Inst, Error> {
        let mut coeffs = Array1::zeros(reg.total());

        for (name, s) in self.reactants {
            coeffs[reg.index(&name)] -= s;
        }

        for (name, s) in self.products {
            coeffs[reg.index(&name)] += s;
        }

        Ok(Self::Inst::new(self.rate.reg(reg)?, coeffs))
    }
}
