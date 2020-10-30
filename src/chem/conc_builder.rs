//! Concentration builder.

use crate::{
    err::Error,
    ord::{Name, Register},
};
use ndarray::Array1;

/// Loadable concentration structure.
pub type ConcBuilder = Vec<(String, f64)>;

impl Name for ConcBuilder {
    type Inst = Array1<f64>;

    #[inline]
    fn reg(self, reg: &Register) -> Result<Self::Inst, Error> {
        let mut cs = Array1::zeros(reg.total());

        for (name, c) in self {
            cs[reg.index(&name)] += c;
        }

        Ok(cs)
    }
}
