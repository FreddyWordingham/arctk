//! Rate building structure.

use crate::{
    chem::Rate,
    err::Error,
    ord::{Name, Register},
};
use arctk_attr::load;

/// Rate of reaction builder.
#[load]
pub struct RateLinker(f64, Vec<(Name, f64)>);

// /// Get a list of all species involved in the rate calculation.
// #[inline]
// #[must_use]
// pub fn names(&self) -> Vec<String> {
//     let mut names = Vec::new();
//     for &(ref n, _) in &self.1 {
//         names.push(n.to_string());
//     }

//     names.sort();
//     names.dedup();

//     names
// }
