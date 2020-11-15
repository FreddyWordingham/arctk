//! Startup parameters file.

use crate::{err::Error, file::Build, geom::MeshBuilder, ord::Set, sim::mcrt::ParametersSetup};
use arctk_attr::load;
use std::path::Path;

/// Parameter builder structure.
#[load]
pub struct ParametersBuilder {
    /// Surfaces.
    surfs: Set<MeshBuilder>,
}

impl Build for ParametersBuilder {
    type Inst = ParametersSetup;

    #[inline]
    fn build(self, in_dir: &Path) -> Result<ParametersSetup, Error> {
        let surfs = self.surfs.build(in_dir)?;

        Ok(ParametersSetup::new(surfs))
    }
}
