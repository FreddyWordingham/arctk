//! Startup parameters file.

use crate::{
    chem::{Concentrations, ReactionLinker},
    ord::Set,
};
use arctk_attr::load;

/// Parameter builder structure.
/// Holds paths to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Initial concentrations.
    pub concs: Concentrations,
    /// Reactions.
    pub reacts: Set<ReactionLinker>,
}

// impl Build for ParametersBuilder {
//     type Inst = ParametersLinker;

//     #[inline]
//     fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
//         Ok(Self::Inst::new())
//     }
// }
