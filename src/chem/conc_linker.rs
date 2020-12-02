//! Concentration builder structure.

use crate::ord::Name;
// use ndarray::Array1;

/// Loadable concentration structure.
pub type Concentrations = Vec<(Name, f64)>;

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
