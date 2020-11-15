//! Startup parameters file.

use crate::{access, geom::Mesh};

/// Functional input parameters.
pub struct Parameters {
    /// Surfaces.
    surfs: Vec<Mesh>,
}

impl Parameters {
    access!(surfs, Vec<Mesh>);

    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn new(surfs: Vec<Mesh>) -> Self {
        debug_assert!(!surfs.is_empty());

        Self { surfs }
    }
}
