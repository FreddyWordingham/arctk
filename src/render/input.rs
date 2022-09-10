//! Program runtime.

use crate::{
    dom::Tree,
    render::{Attribute, Settings, Shader},
};

/// Program runtime data.
pub struct Input<'a> {
    /// Simulation settings.
    pub settings: Settings,
    /// Aesthetic settings.
    pub shader: Shader<'a>,
    /// Scene hierarchy.
    pub tree: Tree<'a, Attribute<'a>>,
}

impl<'a> Input<'a> {
    /// Construct a new instance.
    #[inline]
    #[must_use]
    pub const fn new(settings: Settings, shader: Shader<'a>, tree: Tree<'a, Attribute>) -> Self {
        Self {
            settings,
            shader,
            tree,
        }
    }
}
