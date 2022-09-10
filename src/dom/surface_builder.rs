//! Surface loader.

use serde::Deserialize;
use std::collections::HashMap;

use crate::{dom::Surface, geom::Mesh};

/// Surface parameterisation.
#[derive(Clone, Deserialize)]
pub struct SurfaceBuilder(
    /// Mesh name.
    pub String,
    /// Attribute name.
    pub String,
);

impl SurfaceBuilder {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub fn build<'a, T>(
        self,
        meshes: &HashMap<String, Mesh>,
        attributes: &'a HashMap<String, T>,
    ) -> Surface<'a, T> {
        Surface::new(meshes[&self.0].clone(), &attributes[&self.1])
    }
}
