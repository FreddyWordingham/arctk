//! Reaction linker structure.

use crate::{
    chem::{RateLinker, Reaction},
    err::Error,
    fmt_report, fmt_reports,
    ord::{Link, Name, Set},
};
use arctk_attr::file;
use ndarray::Array1;
use std::fmt::{Display, Formatter};

/// Reaction linker.
#[file]
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
    fn link(self, reg: &'a mut Set<usize>) -> Result<Self::Inst, Error> {
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

impl Display for ReactionLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        fmt_report!(
            fmt,
            self.rate,
            &format!("rate ([C]^{} s^-1)", -self.rate.order())
        );

        let mut rs = Vec::with_capacity(self.reactants.len());
        for &(ref c, m) in &self.reactants {
            rs.push(format!("{}{}", m, c));
        }
        fmt_reports!(fmt, rs, "reactants");

        let mut ps = Vec::with_capacity(self.products.len());
        for &(ref c, m) in &self.products {
            ps.push(format!("{}{}", m, c));
        }
        fmt_reports!(fmt, ps, "products");

        Ok(())
    }
}
