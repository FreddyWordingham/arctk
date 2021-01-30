//! Loadable parameters.

use crate::sim::babbage::OperationBuilderLoader;
use arctk_attr::file;

/// Loadable runtime parameters.
#[file]
pub struct Parameters {
    /// Operations to perform, and their output path.
    pub ops: Vec<(String, OperationBuilderLoader)>,
}
