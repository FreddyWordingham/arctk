//! Startup parameters file.

use crate::chem::Concentrations;
use arctk_attr::load;

/// Parameter builder structure.
/// Holds paths to data still on the disk.
#[load]
pub struct ParametersBuilder {
    /// Initial concentrations.
    pub concs: Concentrations,
    // /// Reactions.
    // reacts: Vec<Reaction>,
}

// impl Build for ParametersBuilder {
//     type Inst = ParametersLinker;

//     #[inline]
//     fn build(self, in_dir: &Path) -> Result<Self::Inst, Error> {
//         Ok(Self::Inst::new())
//     }
// }
