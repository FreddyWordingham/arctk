//! Concentration builder implementation.

use crate::{
    kinetics::{Name, Register},
    Error, Group,
};
use ndarray::Array1;

/// Loadable concentration structure.
pub type ConcBuilder = Vec<(Group, f64)>;

impl Name for ConcBuilder {
    type Inst = Array1<f64>;

    #[inline]
    fn register(self, reg: &Register) -> Result<Self::Inst, Error> {
        let mut concs = Array1::zeros(reg.names().len());

        for (name, c) in self {
            concs[reg.index(&name)] += c;
        }

        Ok(concs)
    }
}
