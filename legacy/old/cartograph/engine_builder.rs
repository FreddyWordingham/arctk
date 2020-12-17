//! Engine selection.

use crate::{
    file::Build,
    sim::cartograph::{engine, Engine},
};
use arctk_attr::load;
use std::path::Path;

/// Engine selection.
#[load]
#[derive(Clone)]
pub enum EngineBuilder {
    /// Basic sampling engine.
    Basic,
}

impl Build for EngineBuilder {
    /// Retrieve a handle to the engine function.
    #[inline]
    #[must_use]
    pub fn build(self, _in_dir: &Path) -> Engine {
        match self {
            Self::Basic => engine::basic::sample,
        }
    }
}
