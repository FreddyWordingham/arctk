//! Reaction linker structure.

use crate::{
    chem::{RateLinker, Reaction},
    err::Error,
    ord::{Link, Name, Set},
};
use arctk_attr::load;
use ndarray::Array1;

/// Reaction linker.
#[load]
pub struct ReactionLinker {
    /// Reaction rate.
    rate: RateLinker,
    /// List of each reactant species and its stoichiometric coefficient.
    reactants: Vec<(Name, f64)>,
    /// List of each product species and its stoichiometric coefficient.
    products: Vec<(Name, f64)>,
}

impl<'a> Link<'a, usize> for ReactionLinker {
    type Inst = Reaction;

    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        let mut names = Vec::with_capacity(self.reactants.len() + self.products.len());
        for &(ref name, ref _x) in &self.reactants {
            names.push(name.clone());
        }
        for &(ref name, ref _x) in &self.products {
            names.push(name.clone());
        }

        names.append(&mut self.rate.requires());

        names
    }

    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        let mut coeffs = Array1::zeros(reg.len());

        for (name, s) in self.reactants {
            coeffs[*reg
                .get(&name)
                .unwrap_or_else(|| panic!("Failed to link reaction-index: {}", name))] -= s;
        }

        for (name, s) in self.products {
            coeffs[*reg
                .get(&name)
                .unwrap_or_else(|| panic!("Failed to link reaction-index: {}", name))] += s;
        }

        Ok(Reaction::new(self.rate.link(reg)?, coeffs))
    }
}
