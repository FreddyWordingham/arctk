//! Parameters setup file.

use crate::{
    geom::Mesh,
    ord::Set,
    sim::mcrt::{Catalogue, Parameters},
};

/// Named setup parameters.
pub struct ParametersSetup {
    /// Surfaces.
    surfs: Set<Mesh>,
}

impl ParametersSetup {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(surfs: Set<Mesh>) -> Self {
        Self { surfs }
    }

    /// Setup the final parameters structure.
    #[inline]
    #[must_use]
    pub fn setup(self) -> (Parameters, Catalogue) {
        let (surfs, surfs_reg) = self.surfs.reg();

        let cat = Catalogue::new(surfs_reg);
        let params = Parameters::new(surfs);

        (params, cat)
    }
}
