//! Reactor linking structure.

use crate::{
    chem::{ReactionLinker, Reactor},
    err::Error,
    fmt_report,
    ord::{Link, Name, Set},
};
use arctk_attr::file;
use ndarray::{Array1, Array2};
use std::fmt::{Display, Formatter};

/// Reactor linking structure.
#[file]
pub struct ReactorLinker(Vec<ReactionLinker>);

impl<'a> Link<'a, usize> for ReactorLinker {
    type Inst = Reactor;

    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        // self.reacts.requires()
        self.0
            .iter()
            .map(|v| v.requires())
            .collect::<Vec<_>>()
            .into_iter()
            .flatten()
            .collect()
    }

    #[inline]
    fn link(self, reg: &'a mut Set<usize>) -> Result<Self::Inst, Error> {
        let mut rates = Vec::with_capacity(self.0.len());
        let mut coeffs = Array2::zeros([self.0.len(), reg.len()]);
        for (mut coeff_set, react) in coeffs.outer_iter_mut().zip(self.0) {
            let (r, cs) = react.link(reg)?.components();
            rates.push(r);
            coeff_set += &cs;
        }

        Ok(Reactor::new(Array1::from(rates), coeffs))
    }
}

impl Display for ReactorLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(fmt, "...")?;
        for reaction in &self.0 {
            fmt_report!(fmt, reaction, "->");
        }
        Ok(())
    }
}
