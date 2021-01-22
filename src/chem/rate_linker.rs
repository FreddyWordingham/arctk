//! Rate building structure.

use crate::{
    chem::Rate,
    err::Error,
    fmt_report, fmt_reports,
    ord::{Link, Name, Set},
};
use arctk_attr::file;
use std::fmt::{Display, Formatter};

/// Rate of reaction builder.
#[file]
pub struct RateLinker(f64, Vec<(Name, f64)>);

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
        writeln!(fmt, "...")?;
        let power = self.1.len();
        fmt_report!(fmt, self.0, &format!("rate ([C]^-{} s^-1)", power));

        let mut orders = Vec::with_capacity(power);
        for &(c, m) in &self.1 {
            orders.push(format!("[{}]^{}", c, m));
        }
        fmt_reports!(fmt, orders, "orders");

        Ok(())
    }
}
