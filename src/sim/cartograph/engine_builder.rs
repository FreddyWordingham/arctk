//! Engine selection.

use crate::sim::cartograph::{engine, Engine};
use arctk_attr::load;

/// Engine selection.
#[load]
#[derive(Clone)]
pub enum EngineBuilder {
    /// Basic sampling engine.
    Basic,
}

impl EngineBuilder {
    /// Retrieve a handle to the engine function.
    #[inline]
    #[must_use]
    pub fn build(self) -> Engine {
        match self {
            Self::Basic => engine::basic::sample,
        }
    }
}
