//! Rate building structure.

use crate::{
    chem::Rate,
    err::Error,
    ord::{Link, Name, Set},
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Rate of reaction builder.
#[file]
pub struct RateLinker(f64, Vec<(Name, f64)>);

impl RateLinker {
    /// Get the total order of the reaction.
    #[inline]
    #[must_use]
    pub fn order(&self) -> f64 {
        let mut p = 0.0;
        for (_, c) in &self.1 {
            p += c;
        }
        p
    }
}

impl<'a> Link<'a, usize> for RateLinker {
    type Inst = Rate;

    #[inline]
    #[must_use]
    fn requires(&self) -> Vec<Name> {
        let mut names = Vec::with_capacity(self.1.len());
        for &(ref name, ref _x) in &self.1 {
            names.push(name.clone());
        }

        names
    }

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

impl Display for RateLinker {
    #[inline]
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", self.0)?;

        if !self.1.is_empty() {
            write!(fmt, " *")?;
            for (c, m) in &self.1 {
                write!(fmt, " {}^{}", c, m)?;
            }
        }

        Ok(())
    }
}
