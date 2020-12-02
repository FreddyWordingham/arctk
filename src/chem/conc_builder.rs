//! Concentration builder structure.

use crate::{
    chem::Species,
    err::Error,
    ord::{Link, Set},
};
use ndarray::Array1;

/// Loadable concentration structure.
pub type ConcBuilder = Vec<(String, f64)>;

// impl<'a> Link<'a> for ConcBuilder {
//     type Inst = Array1<f64>;

//     #[inline]
//     fn build(self, _in_dir: &Path) -> Result<Self::Inst, Error> {
//         let mut cs = Array1::zeros(reg.total());

//         for (name, c) in self {
//             cs[reg.index(&name)] += c;
//         }

//         Ok(cs)
//     }
// }

// impl<'a> Link<'a, Species> for ConcBuilder {
//     type Inst = Array1<f64>;

//     #[inline]
//     fn requires(&self) -> Vec<String> {
//         let mut names = Vec::with_capacity(self.len());
//         for (n, _) in self {
//             names.push(n.clone());
//         }

//         names
//     }

//     // #[inline]
//     // fn link(self, specs_reg: &'a Set<Species>) -> Result<Self::Inst, Error> {
//     //     let concs = Array1::zeros(specs.len());
//     //     for (name, c) in self {
//     //         concs[specs.index_of(name)] += c;
//     //     }

//     //     Ok(concs)
//     // } // Use register?
// }
