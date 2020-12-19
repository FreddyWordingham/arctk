//! Reactor linking structure.

use crate::{
    chem::{ReactionLinker, Reactor},
    err::Error,
    ord::{Link, Name, Set},
};
use arctk_attr::file;
use ndarray::{Array1, Array2};

/// Reactor linking structure.
#[file]
pub struct ReactorLinker {
    /// Reactions.
    reacts: Vec<ReactionLinker>,
}

impl<'a> Link<'a, usize> for ReactorLinker {
    type Inst = Reactor;

    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        // self.reacts.requires()
        self.reacts
            .iter()
            .map(|v| v.requires())
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect()
    }

    #[inline]
    fn link(self, reg: &'a Set<usize>) -> Result<Self::Inst, Error> {
        let mut rates = Vec::with_capacity(self.reacts.len());
        let mut coeffs = Array2::zeros([self.reacts.len(), reg.len()]);
        for (mut coeff_set, react) in coeffs.outer_iter_mut().zip(self.reacts) {
            let (r, cs) = react.link(reg)?.components();
            rates.push(r);
            coeff_set += &cs;
        }

        Ok(Reactor::new(Array1::from(rates), coeffs))
    }
}

// Self {
//     rates: Array1::from(rates),
//     coeffs,
// }
